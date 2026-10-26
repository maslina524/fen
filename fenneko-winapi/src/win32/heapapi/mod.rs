use core::{marker::PhantomData, ptr::NonNull};

use crate::{Handle, raw::HANDLE};

import!(GetProcessHeap, HeapAlloc, HeapFree, HeapReAlloc);

pub mod HeapTag {
    pub struct Alloc;
    pub struct Free;
    pub struct ReAlloc;
}

#[derive(Clone, Copy)]
pub struct HeapFlags<Tag> {
    val: u32,
    _marker: PhantomData<Tag>,
}

impl<Tag> HeapFlags<Tag> {
    const HEAP_GENERATE_EXCEPTIONS: u32 = 0b100;
    const HEAP_NO_SERIALIZE: u32 = 0b1;
    const HEAP_REALLOC_IN_PLACE_ONLY: u32 = 0b10000;
    const HEAP_ZERO_MEMORY: u32 = 0b1000;

    #[must_use]
    pub const fn build(self) -> u32 {
        self.val
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct HeapPtr<T: Sized>(NonNull<T>);

impl<T> HeapPtr<T> {
    /// # Safety
    ///
    /// `ptr` must be non-null and point to a live allocation obtained
    /// from the same Windows heap that will later be used to free
    /// it. If `T` has alignment stricter than the heap
    /// guarantees, the caller must ensure the pointer is properly
    /// aligned for `T`.
    pub const unsafe fn new(ptr: *mut T) -> Self {
        // SAFETY: null == does not point to the heap, read Safety
        let nonnull = unsafe { NonNull::new_unchecked(ptr) };
        Self(nonnull)
    }

    #[must_use]
    pub const fn as_non_null(&self) -> NonNull<T> {
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
        let nonnull = self.as_non_null().cast::<U>();
        HeapPtr(nonnull)
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct HeapHandle(HANDLE);

impl Handle for HeapHandle {
    fn from_raw(val: HANDLE) -> Self {
        Self(val)
    }

    fn as_raw(&self) -> HANDLE {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use core::ptr;

    use super::*;

    #[test]
    fn alloc_test() {
        let handle = GetProcessHeap();

        let ptr = HeapAlloc(handle, HeapFlags::alloc(), 1).expect("HeapAlloc return null pointer");

        // SAFETY: We write a number into the memory allocated by
        // `HeapAlloc`
        unsafe { ptr::write(ptr.as_ptr(), i32::MAX) };

        let ret = HeapFree(handle, HeapFlags::free(), ptr);
        assert!(ret.is_ok());
    }

    #[test]
    fn realloc_test() {
        let handle = GetProcessHeap();

        let mut ptr =
            HeapAlloc(handle, HeapFlags::alloc(), 1).expect("HeapAlloc return null pointer");

        // SAFETY: We write a number into the memory allocated by
        // `HeapAlloc`
        unsafe { ptr::write(ptr.as_ptr(), i32::MAX) };

        ptr = HeapReAlloc(handle, HeapFlags::realloc(), ptr, 2)
            .expect("HeapReAlloc return null pointer");

        // SAFETY: We write a number into the memory allocated by
        // `HeapReAlloc`
        unsafe { ptr::write(ptr.as_ptr().cast::<i32>().add(1), i32::MAX / 2) };

        // SAFETY: We check whether the old data was preserved
        // correctly and the new data was written after `HeapReAlloc`
        unsafe {
            assert_eq!(ptr::read(ptr.as_ptr().cast::<i32>()), i32::MAX);
            assert_eq!(ptr::read(ptr.as_ptr().cast::<i32>().add(1)), i32::MAX / 2);
        }

        let ret = HeapFree(handle, HeapFlags::free(), ptr);
        assert!(ret.is_ok());
    }

    #[test]
    fn alloc_flags_test() {
        let handle = GetProcessHeap();

        let ptr: HeapPtr<i8> = HeapAlloc(handle, HeapFlags::alloc().zero_memory(), 1024)
            .expect("HeapAlloc return null pointer");

        for i in 0..1024 {
            // SAFETY: We check whether all 1024 bytes are zeroed as
            // the flag promises
            assert_eq!(unsafe { ptr::read(ptr.as_ptr().add(i)) }, 0i8);
        }

        let ret = HeapFree(handle, HeapFlags::free(), ptr);
        assert!(ret.is_ok());
    }
}
