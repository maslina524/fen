use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::display_for_err;
use crate::os::windows::*;
use crate::os::error::ErrorCode;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FsError {
    FileNotFound = 2,
    PathNotFound = 3,
    AccessDenied = 5,
    SharingViolation = 32,
    InvalidParameter = 87,
    DirAlreadyExists = 183
}

impl From<u32> for FsError {
    fn from(value: u32) -> Self {
        match value {
            2   => Self::FileNotFound,
            3   => Self::PathNotFound,
            5   => Self::AccessDenied,
            32  => Self::SharingViolation,
            87  => Self::InvalidParameter,
            183 => Self::DirAlreadyExists,
            _ => unreachable!("unknown fs error")
        }
    }
}

display_for_err!(FsError);

pub type FsResult<T> = Result<T, FsError>;


const FILE_ATTRIBUTE_READONLY: u32 = 1;
const FILE_ATTRIBUTE_HIDDEN: u32 = 2;

pub fn create_dir(path: impl Into<String>, hidden: bool) -> FsResult<()> {
    let path_str = path.into();
    let wide: Vec<u16> = path_str.encode_utf16().chain(Some(0)).collect();

    let result = unsafe { CreateDirectoryW(
        wide.as_ptr(), 
        core::ptr::null()
    ) };

    if result == 0 {
        let error = ErrorCode::last();
        return Err(FsError::from(error.code()));
    }

    if hidden {
        let result = unsafe { SetFileAttributesW(
            wide.as_ptr(), 
            FILE_ATTRIBUTE_HIDDEN | FILE_ATTRIBUTE_READONLY
        ) };

        if result == 0 {
            let error = ErrorCode::last();
            return Err(FsError::from(error.code()));
        }
    }

    Ok(())
}