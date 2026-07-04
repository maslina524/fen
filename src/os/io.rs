use core::ffi::c_void;
use core::sync::atomic::{AtomicPtr, Ordering};

use alloc::vec::Vec;
use alloc::boxed::Box;

use crate::os::windows::*;

pub const CP_UTF8: u32 = 65001;
pub const INVALID_HANDLE_VALUE: *mut c_void = -1 as isize as *mut c_void;
pub const STDOUT_HANDLE: u32 = 0xFFFFFFF5;
pub const STDERR_HANDLE: u32 = 0xFFFFFFF4;

static IO_PTR: AtomicPtr<Io> = AtomicPtr::new(core::ptr::null_mut());

pub fn get_io() -> &'static Io {
    let mut ptr = IO_PTR.load(Ordering::Acquire);
    if ptr.is_null() {
        let new_io = Box::new(Io::new());
        let new_ptr = Box::into_raw(new_io);
        match IO_PTR.compare_exchange(
            core::ptr::null_mut(),
            new_ptr,
            Ordering::Release,
            Ordering::Acquire,
        ) {
            Ok(_) => ptr = new_ptr,
            Err(existing) => {
                unsafe { drop(Box::from_raw(new_ptr)); }
                ptr = existing;
            }
        }
    }
    unsafe { &*ptr }
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
        let in_buf = &Vec::from(string)[..];
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