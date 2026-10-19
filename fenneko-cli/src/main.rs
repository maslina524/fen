#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![warn(clippy::missing_safety_doc, clippy::undocumented_unsafe_blocks)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "There's no point in them, it will make the code cleaner"
)]
#![allow(clippy::cargo_common_metadata, reason = "Not now")]

#[cfg(not(test))]
mod panic_impl {
    use core::panic::PanicInfo;

    #[panic_handler]
    const fn panic(_info: &PanicInfo) -> ! {
        loop {}
    }
}

mod entry;

use common::prelude::*;

extern crate alloc;

#[global_allocator]
static ALLOCATOR: winapi::allocator::Allocator = winapi::allocator::Allocator;

fn main() -> i32 {
    let args = env::args();
    for arg in args {
        println!("{arg}");
    }
    
    0
}
