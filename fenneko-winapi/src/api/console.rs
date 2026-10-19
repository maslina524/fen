use crate::{
    Handle,
    api::file::{FileHandle, Write},
    raw::{NtCurrentPeb, RTL_USER_PROCESS_PARAMETERS},
};

#[cfg(target_arch = "x86_64")]
const OFFSET: usize = 0x20;
#[cfg(target_arch = "x86")]
const OFFSET: usize = 0x10;

#[derive(Clone, Copy)]
pub enum StdHandle {
    Stdout,
    Stderr,
    Stdin,
}
#[must_use]
pub fn GetStdHandle(typ: StdHandle) -> Option<FileHandle<Write>> {
    let peb = NtCurrentPeb();
    if peb.is_null() {
        return None;
    }

    // SAFETY: We shift by a constant offset,
    // the offset corresponds to unofficial documentation
    let params_ptr: *const RTL_USER_PROCESS_PARAMETERS = unsafe {
        peb
            .byte_add(OFFSET)
            .cast::<*const RTL_USER_PROCESS_PARAMETERS>()
            .read()
    };
    if params_ptr.is_null() {
        return None;
    }
    // SAFETY: The required structure is guaranteed to be located at the pointer
    let params = unsafe { &*params_ptr };

    let val = match typ {
        StdHandle::Stdout => params.StandardOutput,
        StdHandle::Stderr => params.StandardError,
        StdHandle::Stdin => params.StandardInput,
    };

    if val.is_null() {
        return None;
    }

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
