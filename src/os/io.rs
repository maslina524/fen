use core::ffi::c_void;

use crate::os::windows::*;
use crate::sync::OnceLock;

const CP_UTF8: u32 = 65001;
const INVALID_HANDLE_VALUE: *mut c_void = -1 as isize as *mut c_void;
const STDOUT_HANDLE: u32 = 0xFFFFFFF5;
const STDERR_HANDLE: u32 = 0xFFFFFFF4;

static IO_PTR: OnceLock<Io> = OnceLock::new();

pub fn get_io() -> &'static Io {
    IO_PTR.get_or_init(|| {
        Io::new()
    })
}

pub fn set_console_to_utf8() {
    unsafe { SetConsoleOutputCP(CP_UTF8) };
}

pub struct Io {
    pub stdout: HANDLE,
    pub stderr: HANDLE
}

impl Io {
    pub fn from(stdout: HANDLE, stderr: HANDLE) -> Self {
        Self { stdout, stderr }
    }

    pub fn new() -> Self {
        let stdout = unsafe { GetStdHandle(STDOUT_HANDLE) };
        if stdout == INVALID_HANDLE_VALUE { panic!("No console for stdout"); }
        let stderr = unsafe { GetStdHandle(STDERR_HANDLE) };
        if stderr == INVALID_HANDLE_VALUE { panic!("No console for stderr"); }
        
        Self::from(stdout, stderr)
    }

    pub fn raw_print(&self, handle: HANDLE, string: &str) -> u32 {
        let in_buf = string.as_bytes();
        let mut written = 0;

        unsafe { WriteFile(
            handle, 
            in_buf.as_ptr() as *const u8, 
            in_buf.len() as u32, 
            &mut written, 
            core::ptr::null_mut()
        ) };

        written
    }

    pub fn print(&self, string: &str) -> u32 {
        self.raw_print(self.stdout, string)
    }

    pub fn eprint(&self, string: &str) -> u32 {
        self.raw_print(self.stderr, string)
    }
}

unsafe impl Sync for Io {}

#[macro_export]
macro_rules! print {
    () => {};
    ($($tt:tt)*) => {
        $crate::os::io::get_io().print(&alloc::format!($($tt)*));
    }
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::io::get_io().print("\n");
    };
    ($($tt:tt)*) => {
        $crate::os::io::get_io().print(&alloc::format!($($tt)*));
        $crate::os::io::get_io().print("\n");
    }
}

#[macro_export]
macro_rules! eprint {
    () => {};
    ($($tt:tt)*) => {
        $crate::os::io::get_io().eprint(&alloc::format!($($tt)*));
    }
}

#[macro_export]
macro_rules! eprintln {
    () => {
        $crate::io::get_io().eprint("\n");
    };
    ($($tt:tt)*) => {
        $crate::os::io::get_io().eprint(&alloc::format!($($tt)*));
        $crate::os::io::get_io().eprint("\n");
    }
}