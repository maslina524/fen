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

#[cfg(not(test))]
#[cfg(target_os = "windows")]
mod windows_source {
    use core::arch::global_asm;

    use winapi::api::process::ExitProcess;

    global_asm!(
        ".global _start",
        "_start:",
        "    and rsp, -16",
        "    sub rsp, 32",
        "    call fenneko_entry_point",
        "    int 3",
    );

    #[unsafe(no_mangle)]
    extern "C" fn fenneko_entry_point() -> ! {
        let ret = crate::main();
        ExitProcess(ret)
    }
}

const fn main() -> i32 {
    0
}
