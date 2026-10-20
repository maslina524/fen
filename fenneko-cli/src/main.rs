#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
mod panic_impl {
    use core::panic::PanicInfo;

    #[panic_handler]
    const fn panic(_info: &PanicInfo) -> ! {
        loop {}
    }
}

mod actions;
mod render;
mod builtins;
mod entry;

use common::prelude::*;
use common::io::Stdout;

use crate::render::Renderable;

use crate::actions::Action;

extern crate alloc;

#[global_allocator]
static ALLOCATOR: winapi::allocator::Allocator =
    winapi::allocator::Allocator;

fn main() -> i32 {
    let args = env::args();

    let mut stdout = Stdout::new();

    let _ = actions::Help::run(args).unwrap().render_console(&mut stdout);

    0
}
