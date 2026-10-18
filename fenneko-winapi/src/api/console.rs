use crate::{Handle, api::file::{FileHandle, Write}, raw::{NtCurrentPeb, RTL_USER_PROCESS_PARAMETERS}};

#[cfg(target_arch = "x86_64")]
const OFFSET: usize = 0x20;
#[cfg(target_arch = "x86")]
const OFFSET: usize = 0x10;

#[derive(Clone, Copy)]
pub enum StdHandle {
    Stdout,
    Stderr,
    Stdin
}

#[must_use]
pub fn GetStdHandle(typ: StdHandle) -> Option<FileHandle<Write>> {
    // SAFETY: Just function call
    let ptr = NtCurrentPeb();
    if ptr.is_null() {
        return None
    }
    // SAFETY: We shift by a constant offset,
    // the offset corresponds to unofficial documentation
    let params_ptr = unsafe { ptr.add(OFFSET).cast::<RTL_USER_PROCESS_PARAMETERS>() };
    // SAFETY: The required structure is guaranteed to be located at the pointer
    let params = unsafe { &*params_ptr };

    let val = match typ {
        StdHandle::Stdout => params.StandardOutput,
        StdHandle::Stderr => params.StandardError,
        StdHandle::Stdin => params.StandardInput,
    };

    Some(FileHandle::from_raw(val.cast_mut()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_handles_test() {
        GetStdHandle(StdHandle::Stdout).expect("Failed to get std handle");
        GetStdHandle(StdHandle::Stderr).expect("Failed to get std handle");
        GetStdHandle(StdHandle::Stdin).expect("Failed to get std handle");
    }
}