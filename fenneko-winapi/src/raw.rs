#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#![allow(clippy::unreadable_literal)]

use core::{ffi::c_void, mem};

use crate::intrinsic;

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
link!("ntdll" fn NtWriteFile(fileHandle: HANDLE, event: HANDLE, apcRoutine: PIO_APC_ROUTINE, apcContext: PVOID, ioStatusBlock: *mut IO_STATUS_BLOCK, buffer: PVOID, length: ULONG, byteOffset: PLARGE_INTEGER, key: PULONG) -> NTSTATUS);

#[must_use]
pub fn NtCurrentPeb() -> *const Peb {
    // SAFETY: Valid address on `x86_64`
    #[cfg(target_arch = "x86_64")]
    let peb = unsafe { intrinsic::__readgsqword(0x60) };

    // SAFETY: Valid address on `x86`
    #[cfg(target_arch = "x86")]
    let peb = unsafe { intrinsic::__readfsdword(0x30) as u64 };

    peb as *const _
}

pub type HANDLE = *mut c_void;
pub type NTSTATUS = i32;
pub type PIO_APC_ROUTINE = *mut c_void;

// Pointers
pub type PULONG = *const u32;
pub type PLARGE_INTEGER = *const i64;
pub type PVOID = *mut c_void;
pub type LPVOID = *mut c_void;

// Primitives
pub type BYTE = u8;
pub type USHORT = u16;
pub type ULONG = u32;
pub type ULONG_PTR = usize;

// Strings
pub type PWSTR = *const u16;

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

#[repr(C)]
pub struct Peb {
    pub InheritedAddressSpace: BYTE,
    pub ReadImageFileExecOptions: BYTE,
    pub BeingDebugged: BYTE,
    pub BitField: BYTE,
    pub Mutant: PVOID,                 // 0x08
    pub ImageBaseAddress: PVOID,       // 0x10
    pub Ldr: PVOID,                    // 0x18
    pub ProcessParameters: *const RTL_USER_PROCESS_PARAMETERS, // 0x20
    pub SubSystemData: PVOID,          // 0x28
    pub ProcessHeap: PVOID,            // 0x30
    // etc.
}

#[repr(C)]
pub struct UNICODE_STRING {
    pub Length: USHORT,
    pub MaximumLength: USHORT,
    pub Buffer: PWSTR,
}

#[repr(C)]
pub struct CURDIR {
    pub DosPath: UNICODE_STRING,
    pub Handle: HANDLE,
}

#[repr(C)]
pub struct RTL_DRIVE_LETTER_CURDIR {
    pub Flags: USHORT,
    pub Length: USHORT,
    pub TimeStamp: ULONG,
    pub DosPath: UNICODE_STRING,
}

#[repr(C)]
pub struct RTL_USER_PROCESS_PARAMETERS {
    pub MaximumLength: ULONG,
    pub Length: ULONG,
    pub Flags: ULONG,
    pub DebugFlags: ULONG,
    pub ConsoleHandle: PVOID,
    pub ConsoleFlags: ULONG,
    pub StandardInput: PVOID,
    pub StandardOutput: PVOID,
    pub StandardError: PVOID,
    pub CurrentDirectory: CURDIR,
    pub DllPath: UNICODE_STRING,
    pub ImagePathName: UNICODE_STRING,
    pub CommandLine: UNICODE_STRING,
    pub Environment: PVOID,
    pub StartingX: ULONG,
    pub StartingY: ULONG,
    pub CountX: ULONG,
    pub CountY: ULONG,
    pub CountCharsX: ULONG,
    pub CountCharsY: ULONG,
    pub FillAttribute: ULONG,
    pub WindowFlags: ULONG,
    pub ShowWindowFlags: ULONG,
    pub WindowTitle: UNICODE_STRING,
    pub DesktopInfo: UNICODE_STRING,
    pub ShellInfo: UNICODE_STRING,
    pub RuntimeData: UNICODE_STRING,
    pub CurrentDirectores: [RTL_DRIVE_LETTER_CURDIR; 32],
    pub EnvironmentSize: ULONG,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union IO_STATUS_BLOCK_INNER {
    pub Status: NTSTATUS,
    pub Pointer: PVOID,
}

#[repr(C)]
pub struct IO_STATUS_BLOCK {
    pub Union: IO_STATUS_BLOCK_INNER,
    pub Information: ULONG_PTR,
}

impl Default for IO_STATUS_BLOCK {
    fn default() -> Self {
        // SAFETY: All fields can be zeroed
        unsafe { mem::zeroed() }
    }
}
