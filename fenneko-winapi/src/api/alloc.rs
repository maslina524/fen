use core::{
    marker::PhantomData,
    ptr::{self, NonNull},
};

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

#[repr(transparent)]
pub struct HeapPtr<T: Sized>(NonNull<T>);

impl<T> HeapPtr<T> {
    /// # Safety
    ///
    /// The pointer must be guaranteed to point to an initialized part of the heap;
    /// the wrapper is required for safe operation with the allocator functions.
    pub const unsafe fn new(ptr: *mut T) -> Self {
        // SAFETY: Null == does not point to the heap, read Safety
        let nonnull = unsafe { NonNull::new_unchecked(ptr) };
        Self(nonnull)
    }

    #[must_use]
    pub const fn as_nonull(&self) -> NonNull<T> {
        self.0
    }

    #[must_use]
    pub const fn as_ptr(&self) -> *mut T {
        self.0.as_ptr()
    }

    #[must_use]
    pub const fn into_ptr(self) -> *mut T {
        self.0.as_ptr()
    }

    #[must_use]
    pub const fn cast<U>(self) -> HeapPtr<U> {
        let nonnull = self.as_nonull().cast::<U>();
        HeapPtr(nonnull)
    }
}

#[must_use]
pub fn GetProccessHeap() -> HeapHandle {
    // SAFETY: Valid address on x86_64
    #[cfg(target_arch = "x86_64")]
    let peb = unsafe { intrinsic::__readgsqword(0x60) };

    // SAFETY: Valid address on x86
    #[cfg(target_arch = "x86")]
    let peb = unsafe { intrinsic::__readfsdword(0x30) as u64 };

    // SAFETY: Valid address
    let raw = unsafe { ptr::read_unaligned((peb + PEB_OFFSET) as *const HANDLE) };
    HeapHandle::from_raw(raw)
}

#[must_use]
pub fn HeapAlloc(handle: HeapHandle, flags: HeapFlags<Alloc>, bytes: usize) -> HeapPtr<()> {
    // SAFETY: `HeapHandle` guarantees a valid handle, `HeapFlags<Alloc>` restricts to valid flags, safe
    let raw = unsafe { raw::RtlAllocateHeap(handle.as_raw(), flags.build(), bytes) };
    // SAFETY: The pointer was obtained from the heap
    unsafe { HeapPtr::new(raw.cast()) }
}

pub fn HeapFree(handle: HeapHandle, flags: HeapFlags<Free>, ptr: HeapPtr<()>) {
    // SAFETY: `HeapHandle` guarantees a valid handle, `HeapFlags<Free>` restricts to valid flags, safe
    unsafe {
        raw::RtlFreeHeap(handle.as_raw(), flags.build(), ptr.into_ptr().cast());
    }
}

#[must_use]
pub fn HeapReAlloc(
    handle: HeapHandle,
    flags: HeapFlags<ReAlloc>,
    ptr: HeapPtr<()>,
    size: usize,
) -> HeapPtr<()> {
    // SAFETY: `HeapHandle` guarantees a valid handle, `HeapFlags<ReAlloc>` restricts to valid flags, safe
    let raw = unsafe {
        raw::RtlReAllocateHeap(handle.as_raw(), flags.build(), ptr.into_ptr().cast(), size)
    };
    // SAFETY: The pointer was obtained from the heap
    unsafe { HeapPtr::new(raw.cast()) }
}

#[cfg(test)]
mod tests {
    use core::ptr;

    use super::*;

    const SIZE: usize = size_of::<i32>();

    #[test]
    fn alloc_test() {
        let handle = GetProccessHeap();

        let ptr = HeapAlloc(handle, HeapFlags::alloc(), SIZE);

        // SAFETY: We write a number into the memory allocated by Alloc
        unsafe { ptr::write(ptr.as_ptr().cast(), i32::MAX) };

        HeapFree(handle, HeapFlags::free(), ptr);
    }

    #[test]
    fn realloc_test() {
        let handle = GetProccessHeap();

        let mut ptr = HeapAlloc(handle, HeapFlags::alloc(), SIZE);

        // SAFETY: We write a number into the memory allocated by Alloc
        unsafe { ptr::write(ptr.as_ptr().cast(), i32::MAX) };

        ptr = HeapReAlloc(handle, HeapFlags::realloc(), ptr, SIZE * 2);

        // SAFETY: We write a number into the memory allocated by ReAlloc
        unsafe { ptr::write(ptr.as_ptr().cast::<i32>().add(1), i32::MAX / 2) };

        // SAFETY: We check whether the old data was preserved
        // correctly and the new data was written after ReAlloc
        unsafe {
            assert_eq!(ptr::read(ptr.as_ptr().cast::<i32>()), i32::MAX);
            assert_eq!(ptr::read(ptr.as_ptr().cast::<i32>().add(1)), i32::MAX / 2);
        }

        HeapFree(handle, HeapFlags::free(), ptr);
    }

    #[test]
    fn alloc_flags_test() {
        let handle = GetProccessHeap();

        let ptr = HeapAlloc(handle, HeapFlags::alloc().zero_memory(), 1024).cast::<i8>();

        for i in 0..1024 {
            // SAFETY: We check whether all 1024 bytes are zeroed as the flag promises
            assert_eq!(unsafe { ptr::read(ptr.as_ptr().add(i)) }, 0i8);
        }

        HeapFree(handle, HeapFlags::free(), ptr.cast());
    }
}
