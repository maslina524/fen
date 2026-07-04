#![no_std]
#![no_main]

use crate::os::io;

mod os;

extern crate alloc;

#[global_allocator]
static ALLOCATOR: os::noalloc::WinAllocator = os::noalloc::WinAllocator;

#[cfg(not(test))]
mod panic_impl {
    use core::panic::PanicInfo;
    use crate::{os::windows::ExitProcess, println};

    #[panic_handler]
    fn panic(info: &PanicInfo) -> ! {
        let msg = info.message().as_str().unwrap_or("No message for panic");
        println!("panic: {msg}");

        if let Some(loc) = info.location() {
            let line = loc.line();
            let column = loc.column();
            let file_name = loc.file();
            println!("{}:{}:{}", file_name, line, column);
        }
        unsafe { ExitProcess(101) }
    }
}

fn div(a: i32, b: i32) -> i32 {
    a / b
}

#[unsafe(no_mangle)]
extern "C" fn main() -> i32 {
    io::set_console_to_utf8();
    div(10, 0)
}