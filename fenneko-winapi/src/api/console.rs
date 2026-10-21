use crate::{
    Handle,
    api::file::{FileHandle, Write},
    raw::RtlGetCurrentPeb,
};

#[derive(Clone, Copy)]
pub enum StdHandle {
    Stdout,
    Stderr,
    Stdin,
}

/// # Panics
///
/// Panics when it fails to get the PEB, in theory this should NEVER
/// happen and nothing would work without the PEB, so it panics
/// instead of returning ``Result``/``Option``.
#[must_use]
pub fn GetStdHandle(typ: StdHandle) -> Option<FileHandle<Write>> {
    // SAFETY: We always get a valid pointer to the PEB
    let ptr = unsafe { RtlGetCurrentPeb() };
    assert!(!ptr.is_null(), "Failed to get PEB");
    // SAFETY: The PEB is guaranteed to be located at the `ptr`
    let peb = unsafe { &*ptr };

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

    let val = match typ {
        StdHandle::Stdout => params.StandardOutput,
        StdHandle::Stderr => params.StandardError,
        StdHandle::Stdin => params.StandardInput,
    };

    if val.is_null() {
        return None;
    }

    Some(FileHandle::from_raw(val))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_handles_test() {
        GetStdHandle(StdHandle::Stdout)
            .expect("Failed to get std handle");
        GetStdHandle(StdHandle::Stderr)
            .expect("Failed to get std handle");
        GetStdHandle(StdHandle::Stdin)
            .expect("Failed to get std handle");
    }
}
