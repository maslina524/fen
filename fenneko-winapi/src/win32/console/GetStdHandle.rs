use core::{error::Error, fmt, mem::ManuallyDrop};

use crate::{
    Handle as _,
    raw::ntdll,
    win32::{console::ConsoleHandle, fileapi::FileHandle},
};

#[derive(Clone, Copy)]
pub enum StdHandle {
    Stdout,
    Stderr,
    Stdin,
}

#[derive(Debug, Clone, Copy)]
pub struct GetStdError;

impl fmt::Display for GetStdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for GetStdError {}

/// # Errors
///
/// Returns `Err` when an invalid handle is stored in the PEB.
///
/// # Panics
///
/// Panics when it fails to get the PEB, in theory this should NEVER
/// happen and nothing would work without the PEB, so it panics
/// instead of returning ``Result``/``Option``.
pub fn GetStdHandle(typ: StdHandle) -> Result<ConsoleHandle, GetStdError> {
    // SAFETY: We always get a valid pointer to the PEB
    let ptr = unsafe { ntdll::RtlGetCurrentPeb() };
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
        return Err(GetStdError);
    }

    Ok(ManuallyDrop::new(FileHandle::from_raw(val)))
}
