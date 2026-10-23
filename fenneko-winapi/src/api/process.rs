use alloc::string::String;
use core::{hint::spin_loop, slice};

use crate::{
    handle,
    raw::{self, RtlGetCurrentPeb},
};

pub fn ExitProcess(code: i32) -> ! {
    // SAFETY: Passing itself (-1) as the descriptor,
    // called in the binary, safe
    let _ = unsafe { raw::NtTerminateProcess(handle!(-1), code) };

    loop {
        spin_loop();
    }
}

/// # Panics
///
/// Panics when it fails to get the PEB, in theory this should NEVER
/// happen and nothing would work without the PEB, so it panics
/// instead of returning ``Result``/``Option``.
#[must_use]
pub fn GetCommandLine() -> String {
    // SAFETY: We always get a valid pointer to the PEB
    let peb_ptr = unsafe { RtlGetCurrentPeb() };
    assert!(!peb_ptr.is_null(), "Failed to get PEB");
    // SAFETY: The PEB is guaranteed to be located at the `ptr`
    let peb = unsafe { &*peb_ptr };

    // SAFETY: We shift by a constant offset,
    // the offset corresponds to unofficial documentation
    let params_ptr = peb.ProcessParameters;
    assert!(
        !params_ptr.is_null(),
        "Failed to get ProcessParameters from PEB"
    );
    // SAFETY: The required structure is guaranteed to be located at
    // the pointer
    let params = unsafe { &*params_ptr };

    let ptr = params.CommandLine.Buffer;
    let len = params.CommandLine.Length;
    // SAFETY: Slice from the memory area allocated by WinAPI, safe
    let slice = unsafe { slice::from_raw_parts(ptr, len as usize / size_of::<u16>()) };
    String::from_utf16_lossy(slice)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_line_test() {
        let string = GetCommandLine();
        assert!(string.contains("fen"));
    }
}
