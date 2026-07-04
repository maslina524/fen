use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::os::windows::*;
use crate::os::error::ErrorCode;

pub fn create_dir(path: impl Into<String>) -> Result<(), String> {
    let path_str = path.into();
    let wide: Vec<u16> = path_str.encode_utf16().chain(Some(0)).collect();

    let result = unsafe {
        CreateDirectoryW(wide.as_ptr(), core::ptr::null())
    };

    if result == 0 {
        let error = ErrorCode::last();
        return Err(error.to_string());
    }
    
    Ok(())
}