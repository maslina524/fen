use crate::{Handle as _, raw::ntdll, win32::heapapi::HeapHandle};

/// # Panics
///
/// Panics when it fails to get the PEB, in theory this should NEVER
/// happen and nothing would work without the PEB, so it panics
/// instead of returning ``Result``/``Option``.
#[must_use]
pub fn GetProcessHeap() -> HeapHandle {
    // SAFETY: We always get a valid pointer to the PEB
    let ptr = unsafe { ntdll::RtlGetCurrentPeb() };
    assert!(!ptr.is_null(), "Failed to get PEB");
    // SAFETY: The PEB is guaranteed to be located at the `ptr`
    let peb = unsafe { &*ptr };

    let raw = peb.ProcessHeap;
    HeapHandle::from_raw(raw)
}