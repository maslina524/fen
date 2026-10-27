use core::ptr;

use crate::{
    ntstatus::Ntstatus,
    raw::{
        IO_STATUS_BLOCK, OBJECT_ATTRIBUTES,
        ntdll::{self, NtClose},
    },
    unicode::UnicodeString,
    win32::fileapi::{FileAttributes, FileDispose},
};

const FILE_LIST_DIRECTORY: u32 = 1;
const SYNCHRONIZE: u32 = 1 << 20;

const FILE_ATTRIBUTE_DIRECTORY: u32 = 1 << 4;

const FILE_SHARE_READ: u32 = 1;
const FILE_SHARE_WRITE: u32 = 1 << 1;
const FILE_SHARE_DELETE: u32 = 1 << 2;

const FILE_DIRECTORY_FILE: u32 = 1;
const FILE_SYNCHRONOUS_IO_NONALERT: u32 = 1 << 5;

#[derive(Debug, Clone, Copy)]
pub enum CreateError {
    UnicodePathAllocate,
    FailedToCreate(Ntstatus),
}

/// # Errors
///
/// On error returns `CreateDirError`, which is either a Unicode string
/// allocation error or a `NtCreateFile` call error (Ntstatus).
pub fn CreateDirectory(path: &[u16], flags: FileAttributes) -> Result<(), CreateError> {
    let unicode = UnicodeString::from_path(path).ok_or(CreateError::UnicodePathAllocate)?;

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
            FILE_LIST_DIRECTORY | SYNCHRONIZE,
            &raw mut attrs,
            &raw mut io,
            ptr::null(),
            FILE_ATTRIBUTE_DIRECTORY | flags.build(),
            FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
            FileDispose::CreateNew as u32,
            FILE_DIRECTORY_FILE | FILE_SYNCHRONOUS_IO_NONALERT,
            ptr::null_mut(),
            0,
        )
    };
    if !ret.is_success() {
        return Err(CreateError::FailedToCreate(ret));
    }

    // SAFETY: Don't use a wrapper since we're working with a "raw" handle
    unsafe { NtClose(out) };
    Ok(())
}
