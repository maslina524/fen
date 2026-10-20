use core::{marker::PhantomData, ptr::NonNull};

use crate::{
    Handle,
    raw::{self, HANDLE, NtCurrentPeb},
};

pub struct Alloc;
pub struct Free;
pub struct ReAlloc;

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

/// # Panics
///
/// Panics when it fails to get the PEB, in theory this should NEVER
/// happen and nothing would work without the PEB, so it panics
/// instead of returning ``Result``/``Option``.
#[must_use]
pub fn GetProcessHeap() -> HeapHandle {
    let ptr = NtCurrentPeb();
    assert!(!ptr.is_null(), "Failed to get PEB");
    // SAFETY: The PEB is guaranteed to be located at the `ptr`
    let peb = unsafe { &*ptr };

    let raw = peb.ProcessHeap;
    HeapHandle::from_raw(raw)
}

#[must_use]
pub fn HeapAlloc<T: Sized>(
    handle: HeapHandle,
    flags: HeapFlags<Alloc>,
    count: usize,
) -> Option<HeapPtr<T>> {
    let bytes = count.checked_mul(size_of::<T>())?;

    // SAFETY: `HeapHandle` guarantees a valid handle,
    // `HeapFlags<Alloc>` restricts to valid flags, safe
    let raw = unsafe {
        raw::RtlAllocateHeap(handle.as_raw(), flags.build(), bytes)
    };
    if raw.is_null() {
        return None;
    }

    // SAFETY: `ptr` was obtained from the heap
    Some(unsafe { HeapPtr::new(raw.cast()) })
}

/// # Errors
///
/// If memory cannot be freed, the old `ptr`
/// passed to the function is returned in ``Err()``.
pub fn HeapFree<T: Sized>(
    handle: HeapHandle,
    flags: HeapFlags<Free>,
    ptr: HeapPtr<T>,
) -> Result<(), HeapPtr<T>> {
    // SAFETY: `HeapHandle` guarantees a valid handle,
    // `HeapFlags<Free>` restricts to valid flags, safe
    let ret = unsafe {
        raw::RtlFreeHeap(
            handle.as_raw(),
            flags.build(),
            ptr.as_ptr().cast(),
        )
    };
    if ret.into_bool() { Ok(()) } else { Err(ptr) }
}

/// # Errors
///
/// If memory cannot be reallocated, the old `ptr`
/// passed to the function is returned in ``Err()``.
pub fn HeapReAlloc<T: Sized>(
    handle: HeapHandle,
    flags: HeapFlags<ReAlloc>,
    ptr: HeapPtr<T>,
    count: usize,
) -> Result<HeapPtr<T>, HeapPtr<T>> {
    let Some(bytes) = count.checked_mul(size_of::<T>()) else {
        return Err(ptr);
    };

    // SAFETY: `HeapHandle` guarantees a valid handle,
    // `HeapFlags<ReAlloc>` restricts to valid flags, safe
    let raw = unsafe {
        raw::RtlReAllocateHeap(
            handle.as_raw(),
            flags.build(),
            ptr.as_ptr().cast(),
            bytes,
        )
    };
    if raw.is_null() {
        return Err(ptr);
    }

    // SAFETY: The `ptr` was obtained from the heap
    Ok(unsafe { HeapPtr::new(raw.cast()) })
}

#[cfg(test)]
mod tests {
    use core::ptr;

    use super::*;

    #[test]
    fn alloc_test() {
        let handle = GetProcessHeap();

        let ptr = HeapAlloc(handle, HeapFlags::alloc(), 1)
            .expect("HeapAlloc return null pointer");

        // SAFETY: We write a number into the memory allocated by
        // `HeapAlloc`
        unsafe { ptr::write(ptr.as_ptr(), i32::MAX) };

        let ret = HeapFree(handle, HeapFlags::free(), ptr);
        assert!(ret.is_ok());
    }

    #[test]
    fn realloc_test() {
        let handle = GetProcessHeap();

        let mut ptr = HeapAlloc(handle, HeapFlags::alloc(), 1)
            .expect("HeapAlloc return null pointer");

        // SAFETY: We write a number into the memory allocated by
        // `HeapAlloc`
        unsafe { ptr::write(ptr.as_ptr(), i32::MAX) };

        ptr = HeapReAlloc(handle, HeapFlags::realloc(), ptr, 2)
            .expect("HeapReAlloc return null pointer");

        // SAFETY: We write a number into the memory allocated by
        // `HeapReAlloc`
        unsafe {
            ptr::write(
                ptr.as_ptr().cast::<i32>().add(1),
                i32::MAX / 2,
            )
        };

        // SAFETY: We check whether the old data was preserved
        // correctly and the new data was written after `HeapReAlloc`
        unsafe {
            assert_eq!(
                ptr::read(ptr.as_ptr().cast::<i32>()),
                i32::MAX
            );
            assert_eq!(
                ptr::read(ptr.as_ptr().cast::<i32>().add(1)),
                i32::MAX / 2
            );
        }

        let ret = HeapFree(handle, HeapFlags::free(), ptr);
        assert!(ret.is_ok());
    }

    #[test]
    fn alloc_flags_test() {
        let handle = GetProcessHeap();

        let ptr: HeapPtr<i8> =
            HeapAlloc(handle, HeapFlags::alloc().zero_memory(), 1024)
                .expect("HeapAlloc return null pointer");

        for i in 0..1024 {
            // SAFETY: We check whether all 1024 bytes are zeroed as
            // the flag promises
            assert_eq!(
                unsafe { ptr::read(ptr.as_ptr().add(i)) },
                0i8
            );
        }

        let ret = HeapFree(handle, HeapFlags::free(), ptr);
        assert!(ret.is_ok());
    }
}
