use core::hint::spin_loop;

use crate::{handle, raw};

pub fn ExitProcess(code: i32) -> ! {
    // SAFETY: Passing itself (-1) as the descriptor,
    // called in the binary, safe
    let _ = unsafe { raw::NtTerminateProcess(handle!(-1), code) };

    loop {
        spin_loop();
    }
}
