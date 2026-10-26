use core::ptr;

use crate::{
    Handle,
    ntstatus::Ntstatus,
    raw::{HANDLE, IO_STATUS_BLOCK, OBJECT_ATTRIBUTES, ntdll},
    unicode::UnicodeString,
    win32::fileapi::{FileAccess, FileHandle},
};

mod options {
    pub const NON_DIRECTORY_FILE: u32 = 1 << 6;
}

const NT_CREATE: u32 = 2;
const NT_OVERWRITE_IF: u32 = 5;
const NT_OPEN: u32 = 1;
const NT_OPEN_IF: u32 = 3;
const NT_OVERWRITE: u32 = 4;

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
    CreateFile(Ntstatus),
}

#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy)]
pub struct FileFlags(u32);

impl FileFlags {
    const FILE_ATTRIBUTE_ARCHIVE: u32 = 1 << 5;
    const FILE_ATTRIBUTE_ENCRYPTED: u32 = 1 << 14;
    const FILE_ATTRIBUTE_HIDDEN: u32 = 1 << 1;
    const FILE_ATTRIBUTE_NORMAL: u32 = 1 << 7;
    const FILE_ATTRIBUTE_OFFLINE: u32 = 1 << 12;
    const FILE_ATTRIBUTE_READONLY: u32 = 1;
    const FILE_ATTRIBUTE_SYSTEM: u32 = 1 << 2;
    const FILE_ATTRIBUTE_TEMPORARY: u32 = 1 << 8;

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
        Self(self.0 | Self::FILE_ATTRIBUTE_READONLY)
    }

    #[must_use]
    pub const fn hidden(self) -> Self {
        Self(self.0 | Self::FILE_ATTRIBUTE_HIDDEN)
    }

    #[must_use]
    pub const fn system(self) -> Self {
        Self(self.0 | Self::FILE_ATTRIBUTE_SYSTEM)
    }

    #[must_use]
    pub const fn archive(self) -> Self {
        Self(self.0 | Self::FILE_ATTRIBUTE_ARCHIVE)
    }

    #[must_use]
    pub const fn normal(self) -> Self {
        Self(self.0 | Self::FILE_ATTRIBUTE_NORMAL)
    }

    #[must_use]
    pub const fn temp(self) -> Self {
        Self(self.0 | Self::FILE_ATTRIBUTE_TEMPORARY)
    }

    #[must_use]
    pub const fn offline(self) -> Self {
        Self(self.0 | Self::FILE_ATTRIBUTE_OFFLINE)
    }

    #[must_use]
    pub const fn encrypted(self) -> Self {
        Self(self.0 | Self::FILE_ATTRIBUTE_ENCRYPTED)
    }
}

/// # Errors
///
/// On error returns `CreateFileError`, which is either a Unicode string
/// allocation error or a `NtCreateFile` call error (Ntstatus).
#[allow(unused_variables)]
pub fn CreateFile<A: FileAccess>(
    path: &[u16],
    share: FileShare,
    create: FileDispose,
    flags: FileFlags,
) -> Result<FileHandle<A>, CreateFileError> {
    let access_val = A::raw();
    create_file_inner(path, access_val, share, create, flags).map(FileHandle::from_raw)
}

fn create_file_inner(
    path: &[u16],
    access: u32,
    share: FileShare,
    create: FileDispose,
    flags: FileFlags,
) -> Result<HANDLE, CreateFileError> {
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
            flags.build(),
            share as u32,
            create as u32,
            options::NON_DIRECTORY_FILE,
            ptr::null_mut(),
            0,
        )
    };

    if ret.is_success() {
        Ok(out)
    } else {
        Err(CreateFileError::CreateFile(ret))
    }
}
