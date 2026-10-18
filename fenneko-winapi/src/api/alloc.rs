use core::{ffi::c_void, ptr};

use crate::{
    intrinsic,
    raw::{self, HANDLE},
};

#[cfg(target_arch = "x86_64")]
const PEB_OFFSET: u64 = 0x30;
#[cfg(target_arch = "x86")]
const PEB_OFFSET: u64 = 0x18;

#[must_use]
pub fn GetProccessHeap() -> HANDLE {
    // SAFETY: Valid address on x86_64.
    #[cfg(target_arch = "x86_64")]
    let peb = unsafe { intrinsic::__readgsqword(0x60) };

    // SAFETY: Valid address on x86.
    #[cfg(target_arch = "x86")]
    let peb = unsafe { intrinsic::__readfsqword(0x30) };

    unsafe { ptr::read((peb + PEB_OFFSET) as *const HANDLE) }
}

pub fn HeapAlloc(handle: HANDLE, flags: u32, bytes: usize) -> *mut c_void {
    unsafe { raw::RtlAllocateHeap(handle, flags, bytes as u32) }
}

pub fn HeapFree(handle: HANDLE, flags: u32, pointer: *mut c_void) {
    unsafe {
        raw::RtlFreeHeap(handle, flags, pointer);
    }
}

pub fn HeapReAlloc(handle: HANDLE, flags: u32, pointer: *mut c_void, size: usize) -> *mut c_void {
    unsafe { raw::RtlReAllocateHeap(handle, flags, pointer, size) }
}

#[cfg(test)]
mod tests {
    use core::ptr;

    use super::*;

    #[test]
    fn alloc_test() {
        let handle = GetProccessHeap();

        let ptr = HeapAlloc(handle, 0, size_of::<i32>());
        assert!(!ptr.is_null());

        // SAFETY: We check how safe this is
        unsafe { ptr::write(ptr.cast(), i32::MAX) };

        HeapFree(handle, 0, ptr);
    }
}
