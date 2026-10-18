use core::{ffi::c_void, marker::PhantomData, ptr};

use crate::{
    handle::{Handle, HeapHandle},
    intrinsic,
    raw::{self, HANDLE},
};

#[cfg(target_arch = "x86_64")]
const PEB_OFFSET: u64 = 0x30;
#[cfg(target_arch = "x86")]
const PEB_OFFSET: u64 = 0x18;

pub struct Alloc;
pub struct Free;
pub struct ReAlloc;

#[derive(Clone, Copy)]
pub struct HeapFlags<Tag> {
    val: u32,
    _marker: PhantomData<Tag>,
}

impl<Tag> HeapFlags<Tag> {
    const HEAP_NO_SERIALIZE: u32 = 0b1;
    const HEAP_GENERATE_EXCEPTIONS: u32 = 0b100;
    const HEAP_ZERO_MEMORY: u32 = 0b1000;
    const HEAP_REALLOC_IN_PLACE_ONLY: u32 = 0b10000;

    #[must_use]
    pub const fn build(self) -> u32 {
        self.val
    }
}

impl HeapFlags<Alloc> {
    #[must_use]
    pub const fn alloc() -> Self {
        Self {
            val: 0,
            _marker: PhantomData,
        }
    }

    #[must_use]
    pub const fn generate_exceptions(self) -> Self {
        let val = self.val | Self::HEAP_GENERATE_EXCEPTIONS;
        Self {
            val,
            _marker: self._marker,
        }
    }

    #[must_use]
    pub const fn no_serialize(self) -> Self {
        let val = self.val | Self::HEAP_NO_SERIALIZE;
        Self {
            val,
            _marker: self._marker,
        }
    }

    #[must_use]
    pub const fn zero_memory(self) -> Self {
        let val = self.val | Self::HEAP_ZERO_MEMORY;
        Self {
            val,
            _marker: self._marker,
        }
    }
}

impl HeapFlags<Free> {
    #[must_use]
    pub const fn free() -> Self {
        Self {
            val: 0,
            _marker: PhantomData,
        }
    }

    #[must_use]
    pub const fn no_serialize(self) -> Self {
        let val = self.val | Self::HEAP_NO_SERIALIZE;
        Self {
            val,
            _marker: self._marker,
        }
    }
}

impl HeapFlags<ReAlloc> {
    #[must_use]
    pub const fn realloc() -> Self {
        Self {
            val: 0,
            _marker: PhantomData,
        }
    }

    #[must_use]
    pub const fn generate_exceptions(self) -> Self {
        let val = self.val | Self::HEAP_GENERATE_EXCEPTIONS;
        Self {
            val,
            _marker: self._marker,
        }
    }

    #[must_use]
    pub const fn no_serialize(self) -> Self {
        let val = self.val | Self::HEAP_NO_SERIALIZE;
        Self {
            val,
            _marker: self._marker,
        }
    }

    #[must_use]
    pub const fn zero_memory(self) -> Self {
        let val = self.val | Self::HEAP_ZERO_MEMORY;
        Self {
            val,
            _marker: self._marker,
        }
    }

    #[must_use]
    pub const fn in_place_only(self) -> Self {
        let val = self.val | Self::HEAP_REALLOC_IN_PLACE_ONLY;
        Self {
            val,
            _marker: self._marker,
        }
    }
}

#[must_use]
pub fn GetProccessHeap() -> HeapHandle {
    // SAFETY: Valid address on x86_64.
    #[cfg(target_arch = "x86_64")]
    let peb = unsafe { intrinsic::__readgsqword(0x60) };

    // SAFETY: Valid address on x86.
    #[cfg(target_arch = "x86")]
    let peb = unsafe { intrinsic::__readfsqword(0x30) };

    // SAFETY: Valid address
    let raw = unsafe { ptr::read((peb + PEB_OFFSET) as *const HANDLE) };
    HeapHandle::from_raw(raw)
}

#[must_use]
pub fn HeapAlloc(handle: &HeapHandle, flags: HeapFlags<Alloc>, bytes: usize) -> *mut c_void {
    // SAFETY: `HeapHandle` guarantees a valid handle, `HeapFlags<Alloc>` restricts to valid flags, safe
    unsafe { raw::RtlAllocateHeap(handle.as_raw(), flags.build(), bytes as u32) }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn HeapFree(handle: HeapHandle, flags: HeapFlags<Free>, ptr: *mut c_void) {
    // SAFETY: `HeapHandle` guarantees a valid handle, `HeapFlags<Free>` restricts to valid flags, safe
    unsafe {
        raw::RtlFreeHeap(handle.into_raw(), flags.build(), ptr);
    }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn HeapReAlloc(
    handle: &HeapHandle,
    flags: HeapFlags<ReAlloc>,
    ptr: *mut c_void,
    size: usize,
) -> *mut c_void {
    // SAFETY: `HeapHandle` guarantees a valid handle, `HeapFlags<ReAlloc>` restricts to valid flags, safe
    unsafe { raw::RtlReAllocateHeap(handle.as_raw(), flags.build(), ptr, size) }
}

#[cfg(test)]
mod tests {
    use core::ptr;

    use super::*;

    const SIZE: usize = size_of::<i32>();

    #[test]
    fn alloc_test() {
        let handle = GetProccessHeap();

        let ptr = HeapAlloc(&handle, HeapFlags::alloc(), SIZE);
        assert!(!ptr.is_null());

        // SAFETY: We check how safe this is
        unsafe { ptr::write(ptr.cast(), i32::MAX) };

        HeapFree(handle, HeapFlags::free(), ptr);
    }

    #[test]
    fn realloc_test() {
        let handle = GetProccessHeap();

        let mut ptr = HeapAlloc(&handle, HeapFlags::alloc(), SIZE);
        assert!(!ptr.is_null());

        // SAFETY: We check how safe this is
        unsafe { ptr::write(ptr.cast(), i32::MAX) };

        // Realloc
        ptr = HeapReAlloc(&handle, HeapFlags::realloc(), ptr, SIZE * 2);
        assert!(!ptr.is_null());

        // SAFETY: We check how safe this is
        unsafe { ptr::write(ptr.cast::<i32>().add(1), i32::MAX / 2) };

        // Read
        // SAFETY: We check how safe this is
        unsafe {
            assert_eq!(ptr::read(ptr.cast::<i32>()), i32::MAX);
            assert_eq!(ptr::read(ptr.cast::<i32>().add(1)), i32::MAX / 2);
        }

        HeapFree(handle, HeapFlags::free(), ptr);
    }

    #[test]
    fn alloc_flags_test() {
        let handle = GetProccessHeap();

        let ptr = HeapAlloc(&handle, HeapFlags::alloc().zero_memory(), 1024).cast::<i8>();
        assert!(!ptr.is_null());

        for i in 0..1024 {
            // SAFETY: We check how safe this is
            assert_eq!(unsafe { ptr::read(ptr.add(i)) }, 0i8);
        }
    }
}
