use core::{error::Error, fmt, marker::PhantomData};

use crate::{
    Handle as _,
    raw::ntdll,
    win32::heapapi::{Alloc, HeapFlags, HeapHandle, HeapPtr},
};

#[derive(Debug, Clone, Copy)]
pub enum AllocateError {
    UsizeOverflow,
    AllocateHeapFailed,
}

impl fmt::Display for AllocateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for AllocateError {}

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

/// # Errors
///
/// Returns `Err` when it fails to allocate butes in heap or usize overflow:
/// ``count * size_of::<T>()``.
pub fn HeapAlloc<T: Sized>(
    handle: HeapHandle,
    flags: HeapFlags<Alloc>,
    count: usize,
) -> Result<HeapPtr<T>, AllocateError> {
    let bytes = count
        .checked_mul(size_of::<T>())
        .ok_or(AllocateError::UsizeOverflow)?;

    // SAFETY: `HeapHandle` guarantees a valid handle,
    // `HeapFlags<Alloc>` restricts to valid flags, safe
    let raw = unsafe { ntdll::RtlAllocateHeap(handle.as_raw(), flags.build(), bytes) };
    if raw.is_null() {
        return Err(AllocateError::AllocateHeapFailed);
    }

    // SAFETY: `ptr` was obtained from the heap
    Ok(unsafe { HeapPtr::new(raw.cast()) })
}
