#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
mod panic_impl {
    use core::panic::PanicInfo;
    use common::prelude::*;

    #[panic_handler]
    fn panic(info: &PanicInfo) -> ! {
        println!("{}", info.message());
        loop {}
    }
}

mod actions;
mod builtins;
mod entry;
mod options;
mod render;

use common::{io::Stdout, prelude::*};

use crate::{actions::Action, render::Renderable};

extern crate alloc;

#[global_allocator]
static ALLOCATOR: winapi::allocator::Allocator = winapi::allocator::Allocator;

fn help(stdout: &mut Stdout) {
    let _ = actions::Help::run(None).unwrap().render_console(stdout);
}

fn version(stdout: &mut Stdout) {
    let _ = options::Version::run(None).unwrap().render_console(stdout);
}

fn main() -> i32 {
    let mut args = env::args();
    args.next(); // Skip program name

    let mut stdout = Stdout::new();

    let Some(name) = args.next() else {
        help(&mut stdout);
        return 0;
    };

    match &*name {
        "--version" | "-v" => version(&mut stdout),
        _ => help(&mut stdout),
    }

    0
}
