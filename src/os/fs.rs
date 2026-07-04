use core::ffi::c_void;

use alloc::string::String;

use crate::wide;
use crate::os::windows::*;
use crate::os::error::{self, ErrorCode};

const INVALID_HANDLE_VALUE: *mut c_void = -1 as isize as *mut c_void;

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

pub fn create_file(path: impl Into<String>, content: &[u8], len: usize) -> error::Result<()> {
    let path_str = path.into();
    let path_wide: &[u16] = wide!(path_str);

    let handle = unsafe { CreateFileW(
        path_wide.as_ptr(), 
        GENERIC_WRITE, 
        FILE_SHARE_WRITE | FILE_SHARE_READ, 
        core::ptr::null(), 
        CREATE_ALWAYS, 
        0, 
        core::ptr::null_mut()
    ) };
    if handle == INVALID_HANDLE_VALUE {
        let error = ErrorCode::last();
        return Err(error);
    }

    let written = 0;
    let result = unsafe { WriteFile(
        handle, 
        content.as_ptr(), 
        len as u32, 
        written as *mut u32, 
        core::ptr::null_mut()
    ) };
    if result == 0 {
        let error = ErrorCode::last();
        return Err(error);
    }

    let result = unsafe { CloseHandle(handle) };
    if result == 0 {
        let error = ErrorCode::last();
        return Err(error);
    }

    Ok(())
}