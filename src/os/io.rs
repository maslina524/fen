use core::ffi::c_void;
use core::sync::atomic::{AtomicPtr, Ordering};

use alloc::vec::Vec;
use alloc::boxed::Box;

use crate::os::windows::*;

pub const CP_UTF8: u32 = 65001;
pub const INVALID_HANDLE_VALUE: *mut c_void = -1 as isize as *mut c_void;
pub const STDOUT_HANDLE: u32 = 0xFFFFFFF5;

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

pub struct Io {
    pub stdout: HANDLE
}

impl Io {
    pub fn set_console_to_utf8() {
        unsafe { SetConsoleOutputCP(CP_UTF8) };
    }

    pub fn from(stdout: HANDLE) -> Self {
        
        Self { stdout }
    }

    pub fn new() -> Self {
        let stdout = unsafe { GetStdHandle(STDOUT_HANDLE) };
        if stdout == INVALID_HANDLE_VALUE { panic!("No console"); }
        
        Self::from(stdout)
    }

    pub fn print(&self, string: &str) -> u32 {
        let in_buf = &Vec::from(string)[..];
        let mut written = 0;

        unsafe { WriteFile(
            self.stdout, 
            in_buf.as_ptr() as *const u8, 
            in_buf.len() as u32, 
            &mut written, 
            core::ptr::null_mut()
        ) };

        written
    }
}