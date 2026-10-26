use core::{error::Error, fmt};

use winapi::{
    ntstatus::Ntstatus,
    win32::{
        fileapi::FileFlags,
        winbase::{self, CreateDirectory},
    },
};

#[derive(Debug)]
pub enum CreateDirError {
    PathNotFound,
    NameNotFound,
    AccessDenied,
    AccessPending,
    InvalidParam,
    InvalidName,
    DiskFull,
    AlreadyExists,
    NameTooLong,
    NotADir,
    QuotaExceeded,
    NoSystemResource,
    BadPrivilege,
    Unknonw(Ntstatus),
}

impl fmt::Display for CreateDirError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for CreateDirError {}

/// # Panics
///
/// Panics if the path could not be converted to Unicode, this means:
/// - `Path`/`OsStr` doesn't work correctly.
/// - An invalid `&[u16]` is passed to the function.
///
/// # Errors
///
/// Returns `Err` if the directory could not be created.
pub fn create_dir(path: &[u16]) -> Result<(), CreateDirError> {
    const fn map_ntstatus(nt: Ntstatus) -> CreateDirError {
        use crate::raw::win::fs::CreateDirError as CDE;

        match nt.into_inner() {
            0xC000_003A => CDE::PathNotFound,
            0xC000_0034 => CDE::NameNotFound,
            0xC000_0022 => CDE::AccessDenied,
            0xC000_0056 => CDE::AccessPending,
            0xC000_000D => CDE::InvalidParam,
            0xC000_003B | 0xC000_0033 => CDE::InvalidName,
            0xC000_007F => CDE::DiskFull,
            0xC000_0035 => CDE::AlreadyExists,
            0xC000_0106 => CDE::NameTooLong,
            0xC000_0103 => CDE::NotADir,
            0xC000_00C8 => CDE::QuotaExceeded,
            0xC000_009A => CDE::NoSystemResource,
            0xC000_0061 => CDE::BadPrivilege,
            _ => CDE::Unknonw(nt),
        }
    }

    let err = if let Err(e) = CreateDirectory(path, FileFlags::new()) {
        match e {
            winbase::CreateDirError::UnicodePathAllocate => panic!("Bad path"),
            winbase::CreateDirError::CreateDir(nt) => nt,
        }
    } else {
        return Ok(());
    };

    Err(map_ntstatus(err))
}
