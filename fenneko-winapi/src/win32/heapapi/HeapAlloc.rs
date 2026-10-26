use core::marker::PhantomData;

use crate::{Handle as _, raw::ntdll, win32::heapapi::{HeapFlags, HeapHandle, HeapPtr, HeapTag::Alloc}};

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

#[must_use]
pub fn HeapAlloc<T: Sized>(
    handle: HeapHandle,
    flags: HeapFlags<Alloc>,
    count: usize,
) -> Option<HeapPtr<T>> {
    let bytes = count.checked_mul(size_of::<T>())?;

    // SAFETY: `HeapHandle` guarantees a valid handle,
    // `HeapFlags<Alloc>` restricts to valid flags, safe
    let raw = unsafe { ntdll::RtlAllocateHeap(handle.as_raw(), flags.build(), bytes) };
    if raw.is_null() {
        return None;
    }

    // SAFETY: `ptr` was obtained from the heap
    Some(unsafe { HeapPtr::new(raw.cast()) })
}