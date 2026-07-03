#![no_std]
#![no_main]

#[cfg(not(test))]
mod panic_impl {
    use core::panic::PanicInfo;

    #[panic_handler]
    fn panic(_info: &PanicInfo) -> ! {
        loop {}
    }
}

#[unsafe(no_mangle)]
extern "C" fn main() -> i32 {
    0
}