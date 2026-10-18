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

// extern crate alloc;

use core::ffi;

// #[cfg(not(test))]
#[cfg(target_os = "windows")]
#[unsafe(no_mangle)]
const extern "C" fn main() -> ffi::c_int {
    fennec_main() as ffi::c_int
}

const fn fennec_main() -> i32 {
    0
}
