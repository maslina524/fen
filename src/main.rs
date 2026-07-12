#![no_std]
#![no_main]

pub type NoResult = Result<(), Box<dyn core::error::Error>>;

use alloc::{boxed::Box, format};

use crate::os::io;

mod os;
mod actions;

mod toml;
mod sha1;
mod glob;
mod sync;

extern crate alloc;

#[global_allocator]
static ALLOCATOR: os::noalloc::WinAllocator = os::noalloc::WinAllocator;

#[cfg(not(test))]
mod panic_impl {
    use core::panic::PanicInfo;
    use crate::{os::windows::ExitProcess, println};

    #[panic_handler]
    fn panic(info: &PanicInfo) -> ! {
        println!("{}", info.message());
        
        if let Some(loc) = info.location() {
            println!("{}:{}:{}", loc.file(), loc.line(), loc.column());
        }
        
        unsafe { ExitProcess(101) }
    }
}

fn version() -> NoResult {
    let version = option_env!("CARGO_PKG_VERSION").unwrap_or("unknown");
    println!("Fen v{version}");
    Ok(())
}

fn exec_path() -> NoResult {
    let path = os::env::current_exe();
    println!("{path}");
    Ok(())
}

#[unsafe(no_mangle)]
extern "C" fn main() -> i32 {
    io::set_console_to_utf8();

    let argv = os::env::args();
    let action = match argv.get(1) {
        Some(arg) => arg,
        None => return 1
    };

    let raw_argv = &argv[2..];

    let result: NoResult = match action.as_str() {
        "version" | "--version" => version(),
        "--exec-path" => exec_path(),
        "init" => actions::init(),
        "add" => actions::add(raw_argv),
        _ => Err(format!("unknown command `{action}`").into())
    };

    if let Err(err) = result {
        eprintln!("Fen: {err}");
        return 1;
    }

    return 0;
}