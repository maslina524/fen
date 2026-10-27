use core::ptr;

use crate::{
    Handle,
    ntstatus::Ntstatus,
    raw::{HANDLE, IO_STATUS_BLOCK, OBJECT_ATTRIBUTES, ntdll},
    unicode::UnicodeString,
    win32::fileapi::{FileAccess, FileHandle},
};

const NT_CREATE: u32 = 2;
const NT_OVERWRITE_IF: u32 = 5;
const NT_OPEN: u32 = 1;
const NT_OPEN_IF: u32 = 3;
const NT_OVERWRITE: u32 = 4;

const SYNCHRONIZE: u32 = 0x0010_0000;
const SYNC_FLAGS: u32 = 0x0000_0010 | 0x0000_0020;

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum FileShare {
    None = 0,
    Read = 1,
    Write = 2,
    ReadWrite = 3,
    Delete = 4,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum FileDispose {
    CreateNew = NT_CREATE,
    CreateAlways = NT_OVERWRITE_IF,
    OpenExisting = NT_OPEN,
    OpenAlways = NT_OPEN_IF,
    TruncateExisting = NT_OVERWRITE,
}

#[derive(Debug, Clone, Copy)]
pub enum CreateFileError {
    UnicodePathAllocate,
    FailedToCreate(Ntstatus),
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy)]
pub struct FileAttributes(u32);

impl FileAttributes {
    const ARCHIVE: u32 = 0x0000_0020;
    const ENCRYPTED: u32 = 0x0000_4000;
    const HIDDEN: u32 = 0x0000_0002;
    const NORMAL: u32 = 0x0000_0080;
    const OFFLINE: u32 = 0x0000_1000;
    const READONLY: u32 = 0x0000_0001;
    const SYSTEM: u32 = 0x0000_0004;
    const TEMPORARY: u32 = 0x0000_0100;

    #[must_use]
    pub const fn new() -> Self {
        Self(0)
    }

    #[must_use]
    pub const fn build(self) -> u32 {
        self.0
    }

    #[must_use]
    pub const fn readonly(self) -> Self {
        Self(self.0 | Self::READONLY)
    }

    #[must_use]
    pub const fn hidden(self) -> Self {
        Self(self.0 | Self::HIDDEN)
    }

    #[must_use]
    pub const fn system(self) -> Self {
        Self(self.0 | Self::SYSTEM)
    }

    #[must_use]
    pub const fn archive(self) -> Self {
        Self(self.0 | Self::ARCHIVE)
    }

    #[must_use]
    pub const fn normal(self) -> Self {
        Self(self.0 | Self::NORMAL)
    }

    #[must_use]
    pub const fn temp(self) -> Self {
        Self(self.0 | Self::TEMPORARY)
    }

    #[must_use]
    pub const fn offline(self) -> Self {
        Self(self.0 | Self::OFFLINE)
    }

    #[must_use]
    pub const fn encrypted(self) -> Self {
        Self(self.0 | Self::ENCRYPTED)
    }
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy)]
pub struct CreateOptions(u32);

impl CreateOptions {
    const NON_DIRECTORY_FILE: u32 = 0x0000_0040;
    const SYNCHRONOUS_IO_ALERT: u32 = 0x0000_0010;
    const SYNCHRONOUS_IO_NONALERT: u32 = 0x0000_0020;

    #[must_use]
    pub const fn new() -> Self {
        Self(0)
    }

    #[must_use]
    pub const fn build(self) -> u32 {
        self.0
    }

    #[must_use]
    pub const fn non_directory(self) -> Self {
        Self(self.0 | Self::NON_DIRECTORY_FILE)
    }

    #[must_use]
    pub const fn synchronize(self) -> Self {
        Self(self.0 | Self::SYNCHRONOUS_IO_NONALERT)
    }

    #[must_use]
    pub const fn synchronize_alert(self) -> Self {
        Self(self.0 | Self::SYNCHRONOUS_IO_ALERT)
    }
}

/// # Errors
///
/// On error returns `CreateFileError`, which is either a Unicode string
/// allocation error or a `NtCreateFile` call error (Ntstatus).
pub fn CreateFile<A: FileAccess>(
    path: &[u16],
    share: FileShare,
    create: FileDispose,
    attributes: FileAttributes,
    options: CreateOptions,
) -> Result<FileHandle<A>, CreateFileError> {
    let access_val = A::raw();
    create_file_inner(path, access_val, share, create, attributes, options)
        .map(FileHandle::from_raw)
}

fn create_file_inner(
    path: &[u16],
    access: u32,
    share: FileShare,
    create: FileDispose,
    attributes: FileAttributes,
    options: CreateOptions,
) -> Result<HANDLE, CreateFileError> {
    let access = if options.build() & SYNC_FLAGS != 0 {
        access | SYNCHRONIZE
    } else {
        access
    };

    let unicode = UnicodeString::from_path(path).ok_or(CreateFileError::UnicodePathAllocate)?;

    let mut attrs = OBJECT_ATTRIBUTES {
        ObjectName: unicode.as_inner_ptr(),
        ..Default::default()
    };
    let mut io = IO_STATUS_BLOCK::default();
    let mut out = ptr::null_mut();

    // SAFETY: All wrapper structures comply with the `ntdll` documentation
    let ret = unsafe {
        ntdll::NtCreateFile(
            &raw mut out,
            access,
            &raw mut attrs,
            &raw mut io,
            ptr::null(),
            attributes.build(),
            share as u32,
            create as u32,
            options.build(),
            ptr::null_mut(),
            0,
        )
    };

    if ret.is_success() {
        Ok(out)
    } else {
        Err(CreateFileError::FailedToCreate(ret))
    }
}
