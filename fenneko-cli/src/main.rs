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

mod builtins;
mod entry;

use common::prelude::*;

extern crate alloc;

#[global_allocator]
static ALLOCATOR: winapi::allocator::Allocator =
    winapi::allocator::Allocator;

fn main() -> i32 {
    let args = env::args();
    for arg in args {
        println!("{arg}");
    }

    println!(proc::ansi!("[bold]Hello[/bold], World!"));

    0
}
