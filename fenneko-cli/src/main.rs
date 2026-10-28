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
mod output;
mod render;

use common::{io::Stdout, prelude::*};

use crate::{actions::Action, output::Output, render::Renderable};

extern crate alloc;

#[global_allocator]
static ALLOCATOR: winapi::allocator::Allocator = winapi::allocator::Allocator;

fn help(stdout: &mut Stdout) {
    let _ = actions::Help::run(None).unwrap().render_console(stdout);
}

fn main() -> i32 {
    let mut args = env::args();
    args.next(); // Skip program name

    let mut stdout = Stdout::new();

    let Some(name) = args.next() else {
        help(&mut stdout);
        return 0;
    };

    let output = match name.as_ref() {
        "init" => Output::new(actions::Init::run(None)),
        "--version" | "-v" => Output::new(options::Version::run(None)),
        _ => Output::new(actions::Help::run(None)),
    };

    output.print()
}
