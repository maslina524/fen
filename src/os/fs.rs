use core::ffi::c_void;
use core::mem;

use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::vec;

use crate::{println, wide};
use crate::os::windows::*;
use crate::os::error::{self, ErrorCode};

const INVALID_HANDLE_VALUE: *mut c_void = -1 as isize as *mut c_void;
const INVALID_FILE_ATTRIBUTES: u32 = 0xFFFFFFFF;

const GENERIC_WRITE: u32 = 0x40000000;

const FILE_SHARE_READ: u32 = 0x00000001;
const FILE_SHARE_WRITE: u32 = 0x00000002;

const CREATE_ALWAYS: u32 = 2;

const FILE_ATTRIBUTE_DIRECTORY: u32 = 0x00000010;

pub const FILE_ATTRIBUTE_READONLY: u32 = 1;
pub const FILE_ATTRIBUTE_HIDDEN: u32 = 2;

#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    parts: Vec<String>
}

impl Path {
    pub fn from_str(string: &str) -> Self {
        let wide_path = wide!(string);
        let max_path: usize = if string.starts_with(r"\\?\") {
            32_767
        } else {
            260
        };

        let mut buffer = vec![0u16; max_path];

        let len = unsafe { GetFullPathNameW(
            wide_path.as_ptr(),
            max_path as u32, 
            buffer.as_mut_ptr() as *mut u16, 
            core::ptr::null_mut()
        ) };

        let slice = unsafe { core::slice::from_raw_parts(buffer.as_ptr(), len as usize) };
        let mut absolute = String::from_utf16_lossy(slice);

        let mut parts = Vec::new();
        if absolute.starts_with(r"\\?\") {
            parts.insert(0, r"\\?\".to_owned());
            absolute = absolute[4..].to_owned()
        }
        
        parts.extend(absolute.split(r"\").map(|s| s.to_owned()));

        Self { parts }
    }

    pub fn to_utf16_string(&self) -> Vec<u16> {
        wide!(self.to_string())
    }

    pub fn join(mut self, part: &str) -> Self {
        self.parts.push(part.to_owned());
        self
    }

    pub fn is_dir(&self) -> bool {
        let path_wide = self.to_utf16_string();
        let attrs = unsafe { GetFileAttributesW(path_wide.as_ptr()) };
        if attrs == INVALID_FILE_ATTRIBUTES {
            return false
        }
        return (attrs & FILE_ATTRIBUTE_DIRECTORY) == FILE_ATTRIBUTE_DIRECTORY;
    }

    pub fn is_file(&self) -> bool {
        let path_wide = self.to_utf16_string();
        let attrs = unsafe { GetFileAttributesW(path_wide.as_ptr()) };
        if attrs == INVALID_FILE_ATTRIBUTES {
            return false;
        }
        (attrs & FILE_ATTRIBUTE_DIRECTORY) == 0
    }
}

impl core::fmt::Display for Path {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.parts.join(r"\"))
    }
}

impl From<&str> for Path {
    fn from(s: &str) -> Self {
        Path::from_str(s)
    }
}

pub fn create_dir<T: Into<Path>>(path: T) -> error::Result<()> {
    let wide = path.into().to_utf16_string();

    let result = unsafe { CreateDirectoryW(
        wide.as_ptr(), 
        core::ptr::null()
    ) };

    if result == 0 {
        let error = ErrorCode::last();
        return Err(error);
    }

    Ok(())
}

pub fn set_file_attribute<T: Into<Path>>(path: T, attributes: u32) -> error::Result<()> {
    let wide = path.into().to_utf16_string();

    let result = unsafe { SetFileAttributesW(
        wide.as_ptr(), 
        attributes
    ) };

    if result == 0 {
        let error = ErrorCode::last();
        return Err(error);
    }

    Ok(())
}

pub fn create_file<T: Into<Path>>(path: T, content: &[u8], len: usize) -> error::Result<()> {
    let path_wide = path.into().to_utf16_string();

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

pub fn exists<T: Into<Path>>(path: T) -> bool {
    let path_wide = path.into().to_utf16_string();
    let ret = unsafe {
        PathFileExistsW(path_wide.as_ptr())
    };
    ret == 1
}