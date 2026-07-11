use core::fmt::Display;

use alloc::string::String;

use crate::os::windows::{FormatMessageW, GetLastError};

pub type Result<T> = core::result::Result<T, ErrorCode>;

pub enum ErrorType {
    FileNotFound,
    PathNotFound,
    AccessDenied,
    SharingViolation,
    InvalidParameter,
    DirAlreadyExists
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ErrorCode(u32);

impl ErrorCode {
    pub fn new(code: u32) -> Self {
        Self(code)
    }

    pub fn last() -> Self {
        let code = unsafe { GetLastError() };
        Self::new(code)
    }

    pub fn code(&self) -> u32 {
        self.0
    }

    pub fn typ(&self) -> ErrorType {
        match self.code() {
            2   => ErrorType::FileNotFound,
            3   => ErrorType::PathNotFound,
            5   => ErrorType::AccessDenied,
            32  => ErrorType::SharingViolation,
            87  => ErrorType::InvalidParameter,
            183 => ErrorType::DirAlreadyExists,
            _ => unreachable!("unknown error")
        }
    }

    #[track_caller]
    pub fn panic(&self) -> ! {
        panic!("os err: {self}")
    }
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        const FORMAT_MESSAGE_FROM_SYSTEM: u32 = 0x00001000;
        const FORMAT_MESSAGE_IGNORE_INSERTS: u32 = 0x00000200;

        let buf = [0u16; 128];
        let len = unsafe { FormatMessageW(
            FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS, 
            core::ptr::null(), 
            self.0, 
            0, 
            buf.as_ptr() as *mut u16, 
            128, 
            core::ptr::null()
        ) };

        let slice = unsafe { core::slice::from_raw_parts(&buf as *const u16, len as usize) };
        let string = String::from_utf16_lossy(slice).trim().rsplit("\n").collect::<String>();

        write!(f, "{string}")
    }
}

impl core::error::Error for ErrorCode {}