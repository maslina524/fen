use crate::raw::{peb, types as t};

macro_rules! link {
    ($library:literal $($link_name:literal)? $(#[$doc:meta])? fn $($function:tt)*) => (
        #[link(name = $library)]
        unsafe extern "system" {
            $(#[link_name=$link_name])?
            pub fn $($function)*;
        }
    )
}

link!("ntdll" fn NtTerminateProcess(processhandle: t::HANDLE, exitstatus: t::NTSTATUS) -> t::NTSTATUS);
link!("ntdll" fn RtlAllocateHeap(hHeap: t::HANDLE, dwFlags: t::ULONG, dwBytes: usize) -> t::LPVOID);
link!("ntdll" fn RtlFreeHeap(hHeap: t::HANDLE, dwFlags: t::ULONG, lpMem: t::LPVOID) -> t::BOOLEAN);
link!("ntdll" fn RtlReAllocateHeap(hHeap: t::HANDLE, dwFlags: t::ULONG, lpMem: t::LPVOID, size: usize) -> t::LPVOID);
link!("ntdll" fn NtWriteFile(fileHandle: t::HANDLE, event: t::HANDLE, apcRoutine: t::PIO_APC_ROUTINE, apcContext: t::PVOID, ioStatusBlock: *mut t::IO_STATUS_BLOCK, buffer: t::PVOID, length: t::ULONG, byteOffset: t::PLARGE_INTEGER, key: t::PULONG) -> t::NTSTATUS);
link!("ntdll" fn RtlGetCurrentPeb() -> *const peb::PEB);
// TODO: Replace with own implementation using only ntdll
link!("kernel32" fn SetConsoleOutputCP(wCodePageID: t::UINT) -> t::BOOL);
