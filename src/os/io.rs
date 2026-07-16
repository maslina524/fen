use core::ffi::c_void;

use crate::os::windows::*;
use crate::sync::{Mutex, OnceLock};

const CP_UTF8: u32 = 65001;
const KEY_EVENT: u16 = 0x0001;
const INVALID_HANDLE_VALUE: *mut c_void = -1 as isize as *mut c_void;
const STDIN_HANDLE: u32 = 0xFFFFFFF6;
const STDOUT_HANDLE: u32 = 0xFFFFFFF5;
const STDERR_HANDLE: u32 = 0xFFFFFFF4;
const ENABLE_LINE_INPUT: u32 = 0x0004;
const ENABLE_ECHO_INPUT: u32 = 0x0002;

static PRINT_LOCK: Mutex = Mutex::new();
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
    pub stderr: HANDLE,
    pub stdin: HANDLE
}

impl Io {
    pub fn from(stdout: HANDLE, stderr: HANDLE, stdin: HANDLE) -> Self {
        Self { stdout, stderr, stdin }
    }

    pub fn new() -> Self {
        let stdout = unsafe { GetStdHandle(STDOUT_HANDLE) };
        if stdout == INVALID_HANDLE_VALUE { panic!("no console for stdout"); }
        let stderr = unsafe { GetStdHandle(STDERR_HANDLE) };
        if stderr == INVALID_HANDLE_VALUE { panic!("no console for stderr"); }
        let stdin = unsafe { GetStdHandle(STDIN_HANDLE) };
        if stdin == INVALID_HANDLE_VALUE { panic!("no console for stdin"); }
        
        unsafe {
            let mut mode = 0;
            if GetConsoleMode(stdin, &mut mode) != 0 {
                mode &= !(ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT);
                SetConsoleMode(stdin, mode);
            }
        }

        Self::from(stdout, stderr, stdin)
    }

    pub fn raw_print(&self, handle: HANDLE, string: &str) -> u32 {
        PRINT_LOCK.lock();
        let in_buf = string.as_bytes();
        let mut written = 0;

        unsafe { WriteFile(
            handle, 
            in_buf.as_ptr() as *const u8, 
            in_buf.len() as u32, 
            &mut written, 
            core::ptr::null_mut()
        ) };

        PRINT_LOCK.unlock();
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

pub fn get_input() -> Option<char> {
    let mut buf = unsafe { core::mem::zeroed() };
    let mut nums_read = 0;
    let ret = unsafe { ReadConsoleInputW(
        get_io().stdin, 
        &mut buf, 
        1, 
        &mut nums_read
    ) };

    if ret == 0 || nums_read == 0 {
        return None;
    }

    if buf.EventType == KEY_EVENT {
        let ker: KEY_EVENT_RECORD = unsafe { buf.Event.KeyEvent };
        if ker.bKeyDown == 1 {
            let code = unsafe { ker.uChar.UnicodeChar };
            return char::from_u32(code as u32)
        }
    }

    None
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
        $crate::os::io::get_io().print(&alloc::format!("{}\n", alloc::format!($($tt)*)));
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
        $crate::os::io::get_io().eprint(&alloc::format!("{}\n", alloc::format!($($tt)*)));
    }
}