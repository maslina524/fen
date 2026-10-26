use core::marker::PhantomData;

use crate::{
    Handle as _,
    raw::ntdll,
    win32::heapapi::{Free, HeapFlags, HeapHandle, HeapPtr},
};

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
    let ret = unsafe { ntdll::RtlFreeHeap(handle.as_raw(), flags.build(), ptr.as_ptr().cast()) };
    if ret.into_bool() { Ok(()) } else { Err(ptr) }
}
