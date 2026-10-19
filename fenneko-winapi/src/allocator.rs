use core::{
    alloc::{GlobalAlloc, Layout},
    ffi::c_void,
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
};

use crate::{
    Handle,
    api::alloc::{
        GetProcessHeap, HeapAlloc, HeapFlags, HeapFree, HeapHandle, HeapPtr, HeapReAlloc,
    },
};

static HANDLE: AtomicUsize = AtomicUsize::new(0);

fn handle() -> HeapHandle {
    let atomic = HANDLE.load(Relaxed);
    if atomic != 0 {
        let hmodule = atomic as *mut c_void;
        return HeapHandle::from_raw(hmodule);
    }

    let handle = GetProcessHeap();
    let hmodule = handle.as_raw();
    HANDLE.store(hmodule as usize, Relaxed);
    handle
}

pub struct Allocator;

// SAFETY: VERY safe wrappers are used, but it is still
// required that only pointers pointing to the initialized
// part of the heap are passed
unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = HeapAlloc(handle(), HeapFlags::alloc(), layout.size()).expect("Failed to alloc");
        ptr.into_ptr()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        // SAFETY: `ptr` passed to dealloc must point into the initialized part of the heap;
        // using the stable standard library this shouldn't cause problems
        let heapptr = unsafe { HeapPtr::new(ptr) };
        let ret = HeapFree(handle(), HeapFlags::free(), heapptr);
        assert!(ret.is_ok(), "Failed to free");
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let ptr = HeapAlloc(handle(), HeapFlags::alloc().zero_memory(), layout.size())
            .expect("Failed to alloc (zeroed)");
        ptr.into_ptr()
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: `ptr` passed to dealloc must point into the initialized part of the heap;
        // using the stable standard library this shouldn't cause problems
        let heapptr = unsafe { HeapPtr::new(ptr) };
        let ptr = HeapReAlloc(handle(), HeapFlags::realloc(), heapptr, new_size)
            .expect("Failed to realloc");
        ptr.into_ptr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[global_allocator]
    static ALLOCATOR: Allocator = Allocator;

    const N: usize = 2048;

    #[test]
    fn allocator_test() {
        let mut buf = Vec::new();

        for i in 0..=N {
            buf.push(i);
        }

        let sum = buf.into_iter().sum::<usize>();
        assert_eq!(sum, N * (N + 1) / 2);
    }
}
