use crate::{Handle, raw::ntdll};

pub fn CloseHandle<T: Handle>(handle: &mut T) -> bool {
    let ret = unsafe {
        ntdll::NtClose(handle.as_raw())
    };
    NT_SUCCESS!(ret)
}