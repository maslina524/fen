use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::display_for_err;
use crate::os::windows::*;
use crate::os::error::ErrorCode;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FsError {
    DirAlreadyExists = 183,
    PathNotFound = 3
}

impl From<u32> for FsError {
    fn from(value: u32) -> Self {
        match value {
            183 => Self::DirAlreadyExists,
            3 => Self::PathNotFound,
            _ => unreachable!("unknown fs error")
        }
    }
}

display_for_err!(FsError);

pub type FsResult<T> = Result<T, FsError>;

pub fn create_dir(path: impl Into<String>) -> FsResult<()> {
    let path_str = path.into();
    let wide: Vec<u16> = path_str.encode_utf16().chain(Some(0)).collect();

    let result = unsafe {
        CreateDirectoryW(wide.as_ptr(), core::ptr::null())
    };

    if result == 0 {
        let error = ErrorCode::last();
        return Err(FsError::from(error.code()));
    }

    Ok(())
}