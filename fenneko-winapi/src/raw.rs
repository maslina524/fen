#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#![allow(clippy::unreadable_literal)]

use core::ffi::c_void;

macro_rules! link {
    ($library:literal $($link_name:literal)? $(#[$doc:meta])? fn $($function:tt)*) => (
        #[link(name = $library)]
        unsafe extern "system" {
            $(#[link_name=$link_name])?
            pub fn $($function)*;
        }
    )
}

link!("ntdll" fn NtTerminateProcess(processhandle: HANDLE, exitstatus: NTSTATUS) -> NTSTATUS);
link!("ntdll" fn RtlAllocateHeap(hHeap: HANDLE, dwFlags: ULONG, dwBytes: usize) -> LPVOID);
link!("ntdll" fn RtlFreeHeap(hHeap: HANDLE, dwFlags: ULONG, lpMem: LPVOID) -> BOOLEAN);
link!("ntdll" fn RtlReAllocateHeap(hHeap: HANDLE, dwFlags: ULONG, lpMem: LPVOID, size: usize) -> LPVOID);

pub type HANDLE = *mut c_void;
pub type NTSTATUS = i32;

// Pointers
pub type LPVOID = *mut c_void;

// Primitives
pub type ULONG = u32;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct BOOLEAN(u8);

impl BOOLEAN {
    #[must_use]
    pub fn into_bool(self) -> bool {
        self.into()
    }
}

impl Into<bool> for BOOLEAN {
    fn into(self) -> bool {
        self.0 > 0
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct BOOL(i32);

impl BOOL {
    #[must_use]
    pub fn into_bool(self) -> bool {
        self.into()
    }
}

impl Into<bool> for BOOL {
    fn into(self) -> bool {
        self.0 > 0
    }
}
