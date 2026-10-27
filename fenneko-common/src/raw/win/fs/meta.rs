use core::{error::Error, fmt};

use winapi::{
    ntstatus::Ntstatus,
    win32::fileapi::{FileAttributesError, GetFileAttributes, ItemAttributes},
};

pub type Attributes = ItemAttributes;

#[derive(Debug, PartialEq, Eq)]
pub enum AttributesError {
    PathNotFound,
    FileNotFound,
    AccessDenied,
    InvalidHandle,
    NotEnoughMemory,
    NotReady,
    Crc,
    InvalidParam,
    InvalidName,
    Unknown(Ntstatus),
}

impl fmt::Display for AttributesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for AttributesError {}

/// # Errors
///
/// Returns `Err` if failed to get file attributes
///
/// # Panics
///
/// Panics if the path could not be converted to Unicode, this means:
/// - `Path`/`OsStr` doesn't work correctly.
/// - An invalid `&[u16]` is passed to the function
pub fn raw_attributes(path: &[u16]) -> Result<Attributes, AttributesError> {
    const fn map_ntstatus(nt: Ntstatus) -> AttributesError {
        use crate::raw::fs::AttributesError as AE;

        #[allow(clippy::match_single_binding)]
        match nt.into_inner() {
            0xC000_003A => AE::PathNotFound,
            0xC000_0034 => AE::FileNotFound,
            0xC000_0022 => AE::AccessDenied,
            0xC000_0008 => AE::InvalidHandle,
            0xC000_009A | 0xC000_0017 => AE::NotEnoughMemory,
            0xC000_026E | 0xC000_00A3 => AE::NotReady,
            0xC000_009C | 0xC000_003F => AE::Crc,
            0xC000_000D => AE::InvalidParam,
            0xC000_003B | 0xC000_0033 => AE::InvalidName,
            _ => AE::Unknown(nt),
        }
    }

    let items = GetFileAttributes(path).map_err(|e| match e {
        FileAttributesError::UnicodePathAllocate => panic!("Bad path"),
        FileAttributesError::QueryAttributes(nt) => map_ntstatus(nt),
    })?;

    Ok(items)
}
