use core::{error::Error, fmt};

use winapi::{
    ntstatus::Ntstatus,
    win32::{
        fileapi::FileAttributes,
        winbase::{self, CreateDirectory},
    },
};

#[derive(Debug, PartialEq, Eq)]
pub enum CreateError {
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
    Unknown(Ntstatus),
}

impl fmt::Display for CreateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for CreateError {}

/// # Errors
///
/// Returns `Err` if the directory could not be created.
///
/// # Panics
///
/// Panics if the path could not be converted to Unicode, this means:
/// - `Path`/`OsStr` doesn't work correctly.
/// - An invalid `&[u16]` is passed to the function.
pub fn create_dir(path: &[u16], attrs: FileAttributes) -> Result<(), CreateError> {
    const fn map_ntstatus(nt: Ntstatus) -> CreateError {
        use crate::raw::win::fs::CreateError as CE;

        match nt.into_inner() {
            0xC000_003A => CE::PathNotFound,
            0xC000_0034 => CE::NameNotFound,
            0xC000_0022 => CE::AccessDenied,
            0xC000_0056 => CE::AccessPending,
            0xC000_000D => CE::InvalidParam,
            0xC000_003B | 0xC000_0033 => CE::InvalidName,
            0xC000_007F => CE::DiskFull,
            0xC000_0035 => CE::AlreadyExists,
            0xC000_0106 => CE::NameTooLong,
            0xC000_0103 => CE::NotADir,
            0xC000_00C8 => CE::QuotaExceeded,
            0xC000_009A => CE::NoSystemResource,
            0xC000_0061 => CE::BadPrivilege,
            _ => CE::Unknown(nt),
        }
    }

    let err = if let Err(e) = CreateDirectory(path, attrs) {
        match e {
            winbase::CreateError::UnicodePathAllocate => panic!("Bad path"),
            winbase::CreateError::FailedToCreate(nt) => nt,
        }
    } else {
        return Ok(());
    };

    Err(map_ntstatus(err))
}
