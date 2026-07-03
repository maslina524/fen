use core::ffi::c_void;
use core::alloc::{GlobalAlloc, Layout};
use core::sync::atomic::{AtomicPtr, Ordering};

use crate::os::windows::*;

static HEAP_HANDLE: AtomicPtr<c_void> = AtomicPtr::new(core::ptr::null_mut());

unsafe fn get_heap_handle() -> *mut c_void {
    let handle = HEAP_HANDLE.load(Ordering::Relaxed);
    if handle.is_null() {
        let new_handle = unsafe { GetProcessHeap() };
        HEAP_HANDLE.store(new_handle, Ordering::Relaxed);
        new_handle
    } else {
        handle
    }
}

pub struct WinAllocator;

unsafe impl GlobalAlloc for WinAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let handle = unsafe { get_heap_handle() };
        unsafe {
            HeapAlloc(
                handle, 
                0, 
                layout.size()
            ) as *mut u8
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let handle = unsafe { get_heap_handle() };
        unsafe {
            HeapFree(
                handle, 
                0, 
                ptr as *mut c_void
            );
        }
    }
}