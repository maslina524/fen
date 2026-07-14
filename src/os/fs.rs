use core::ffi::c_void;
use core::mem;

use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::vec;
use alloc::format;

use crate::wide;
use crate::os::windows::*;
use crate::os::error::{self, ErrorCode};

const INVALID_HANDLE_VALUE: *mut c_void = -1 as isize as *mut c_void;
const INVALID_FILE_ATTRIBUTES: u32 = 0xFFFFFFFF;

const GENERIC_WRITE: u32 = 0x40000000;
const GENERIC_READ: u32 = 0x80000000;
const FILE_READ_ATTRIBUTES: u32 = 0x80;

const FILE_SHARE_READ: u32 = 0x00000001;
const FILE_SHARE_WRITE: u32 = 0x00000002;
const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x02000000;

const CREATE_ALWAYS: u32 = 2;
const OPEN_EXISTING: u32 = 3;

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

    pub fn current() -> Self {
        let mut buf = [0u16; 32_767];
        let len = unsafe { GetCurrentDirectoryW(
            32_767,
            &mut buf as *mut u16
        ) };
        if len == 0 {
            ErrorCode::last().panic()
        }

        let string = String::from_utf16_lossy(&buf[..len as usize]);
        Self::from_str(&string)
    }

    pub fn normalize_string(&self) -> String {
        let mut absolute = Path::current().parts;
        let mut path = self.parts.clone();

        while absolute.len() > 0 && path.len() > 0 && absolute[0] == path[0] {
            absolute.remove(0);
            path.remove(0);
        }

        format!("{}", path.join("/"))
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

impl From<String> for Path {
    fn from(s: String) -> Self {
        Path::from_str(s.as_str())
    }
}

impl From<&String> for Path {
    fn from(s: &String) -> Self {
        Path::from_str(s.as_str())
    }
}

impl From<&Path> for Path {
    fn from(s: &Path) -> Self {
        s.to_owned()
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

pub fn create_file_all<T: Into<Path>>(path: T, content: &[u8], len: usize) -> error::Result<()> {
    let path = path.into();
    let parts = &path.parts;

    if parts.len() > 1 {
        let mut temp_path = String::new();
        for part in &parts[0..parts.len() - 1] {
            temp_path.push_str(part);
            temp_path.push('\\');

            if !exists(&temp_path) {
                if let Err(e) = create_dir(&temp_path) {
                    if e.code() != 183 {
                        return Err(e);
                    }
                }
            }
        }
    }
    create_file(path, content, len)
}

pub fn exists<T: Into<Path>>(path: T) -> bool {
    let path_wide = path.into().to_utf16_string();
    let ret = unsafe {
        PathFileExistsW(path_wide.as_ptr())
    };
    ret == 1
}

#[derive(Debug, Clone)]
pub enum Item {
    Directory(String),
    File(String)
}

impl Item {
    pub fn is_file(self) -> bool {
        matches!(self, Item::File(_))
    }

    pub fn is_dir(self) -> bool {
        matches!(self, Item::Directory(_))
    }

    pub fn name(self) -> String {
        match self {
            Item::File(n) => n,
            Item::Directory(n) => n
        }
    }
}

pub fn read_dir<T: Into<Path>>(dir: T) -> error::Result<Vec<Item>> {
    let mut ret = Vec::new();
    let base_path = dir.into();
    let search_path = base_path.clone().join("*");
    let search_path_wide = search_path.to_utf16_string();

    let mut data = unsafe { mem::zeroed() };
    let handle = unsafe { FindFirstFileW(search_path_wide.as_ptr(), &mut data) };
    if handle == INVALID_HANDLE_VALUE {
        ErrorCode::last().panic()
    }

    let name = get_name_from_buf(&data.cFileName);
    if name != "." && name != ".." {
        let item = if base_path.clone().join(&name).is_file() {
            Item::File(name)
        } else {
            Item::Directory(name)
        };
        ret.push(item);
    }

    while unsafe { FindNextFileW(handle, &mut data) } == 1 {
        let name = get_name_from_buf(&data.cFileName);
        if name == "." || name == ".." { continue; }
        let item = if base_path.clone().join(&name).is_file() {
            Item::File(name)
        } else {
            Item::Directory(name)
        };
        ret.push(item);
    }

    unsafe { FindClose(handle) };
    Ok(ret)
}

fn get_name_from_buf(name_buf: &[u16]) -> String {
    let mut len = 0;

    while name_buf[len] != 0 { len += 1; }
    let name = String::from_utf16_lossy(&name_buf[..len]);

    return name;
}

pub fn read_to_bytes<T: Into<Path>>(path: T) -> error::Result<Vec<u8>> {
    let path_wide = path.into().to_utf16_string();

    let handle = unsafe { CreateFileW(
        path_wide.as_ptr(), 
        GENERIC_READ, 
        FILE_SHARE_READ, 
        core::ptr::null(), 
        OPEN_EXISTING, 
        0, 
        core::ptr::null_mut()
    ) };
    if handle == INVALID_HANDLE_VALUE {
        let error = ErrorCode::last();
        return Err(error);
    }

    let mut content = Vec::new();
    let mut buf = [0u8; 1024];
    let mut written = 1;
    loop {
        let ret = unsafe { ReadFile(
            handle, 
            buf.as_mut_ptr() as *mut u8,
            1024,
            &mut written,
            core::ptr::null_mut()
        ) };
        if ret == 0 {
            let error = ErrorCode::last();
            return Err(error);
        }
        if written == 0 { break; }
        content.extend(&buf[..written as usize]);
    }
    
    unsafe { CloseHandle(handle) };

    Ok(content)
}

#[derive(Debug, Clone, Default)]
pub struct FileInfo {
    pub size: u32,
    pub ctime_sec: u32,
    pub ctime_nsec: u32,
    pub mtime_sec: u32,
    pub mtime_nsec: u32,
}

pub fn get_file_info<T: Into<Path>>(path: T) -> error::Result<FileInfo> {
    let path_wide = path.into().to_utf16_string();

    let handle = unsafe { CreateFileW(
        path_wide.as_ptr(), 
        FILE_READ_ATTRIBUTES, 
        FILE_SHARE_READ | FILE_SHARE_WRITE, 
        core::ptr::null(), 
        OPEN_EXISTING, 
        FILE_FLAG_BACKUP_SEMANTICS, 
        core::ptr::null_mut()
    ) };
    if handle == INVALID_HANDLE_VALUE {
        let error = ErrorCode::last();
        return Err(error);
    }

    let mut size = 0;
    if (unsafe { GetFileSizeEx(handle, &mut size) }) == 0 {
        let error = ErrorCode::last();
        return Err(error);
    }

    let mut created_win = unsafe { core::mem::zeroed() };
    let mut modified_win = unsafe { core::mem::zeroed() };
    let ret = unsafe { GetFileTime(
        handle,
        &mut created_win, 
        core::ptr::null_mut(), 
        &mut modified_win
    ) };

    unsafe { CloseHandle(handle) };

    let (ctime_sec, ctime_nsec) = win_time_to_sec_and_nsec(created_win);
    let (mtime_sec, mtime_nsec) = win_time_to_sec_and_nsec(modified_win);

    Ok( FileInfo { ctime_sec, ctime_nsec, mtime_sec, mtime_nsec, size: size as u32 } )
}

fn win_time_to_sec_and_nsec(ft: FILETIME) -> (u32, u32) {
    let ft_64 = ((ft.dwHighDateTime as u64) << 32) | (ft.dwLowDateTime as u64);
    
    const EPOCH_DIFF_100NS: u64 = 11_644_473_600_000_000;
    
    let utc_100ns = ft_64 - EPOCH_DIFF_100NS;
    
    let sec = utc_100ns / 10_000_000;
    let nsec = (utc_100ns % 10_000_000) * 100;
    
    (sec as u32, nsec as u32)
}