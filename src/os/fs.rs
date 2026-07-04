use alloc::string::String;

use crate::wide;
use crate::os::windows::*;
use crate::os::error::{self, ErrorCode};

const FILE_ATTRIBUTE_READONLY: u32 = 1;
const FILE_ATTRIBUTE_HIDDEN: u32 = 2;

const GENERIC_WRITE: u32 = 0x40000000;

const FILE_SHARE_READ: u32 = 0x00000001;
const FILE_SHARE_WRITE: u32 = 0x00000002;

const CREATE_ALWAYS: u32 = 2;

pub fn create_dir(path: impl Into<String>, hidden: bool) -> error::Result<()> {
    let path_str = path.into();
    let wide: &[u16] = wide!(path_str);

    let result = unsafe { CreateDirectoryW(
        wide.as_ptr(), 
        core::ptr::null()
    ) };

    if result == 0 {
        let error = ErrorCode::last();
        return Err(error);
    }

    if hidden {
        let result = unsafe { SetFileAttributesW(
            wide.as_ptr(), 
            FILE_ATTRIBUTE_HIDDEN | FILE_ATTRIBUTE_READONLY
        ) };

        if result == 0 {
            let error = ErrorCode::last();
            return Err(error);
        }
    }

    Ok(())
}