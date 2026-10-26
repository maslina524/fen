use core::marker::PhantomData;

use crate::win32::heapapi::HeapFlags;
use crate::win32::heapapi::HeapTag::ReAlloc;
use crate::win32::heapapi::HeapHandle;
use crate::win32::heapapi::HeapPtr;
use crate::raw::ntdll;
use crate::Handle as _;

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
        ntdll::RtlReAllocateHeap(handle.as_raw(), flags.build(), ptr.as_ptr().cast(), bytes)
    };
    if raw.is_null() {
        return Err(ptr);
    }

    // SAFETY: The `ptr` was obtained from the heap
    Ok(unsafe { HeapPtr::new(raw.cast()) })
}