use core::hint::spin_loop;

use crate::raw::ntdll;

pub fn ExitProcess(code: i32) -> ! {
    // SAFETY: Passing itself (-1) as the descriptor,
    // called in the binary, safe
    let _ = unsafe { ntdll::NtTerminateProcess(handle!(-1), code) };

    loop {
        spin_loop();
    }
}