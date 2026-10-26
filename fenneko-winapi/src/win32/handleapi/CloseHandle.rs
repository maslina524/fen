use crate::{Handle, ntstatus::Ntstatus, raw::ntdll};

pub fn CloseHandle<T: Handle>(handle: &mut T) -> Ntstatus {
    // SAFETY: Handle is valid, safe
    unsafe { ntdll::NtClose(handle.as_raw()) }
}
