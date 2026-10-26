use crate::{
    link,
    raw::{peb, types as t},
};

link!("ntdll" fn NtTerminateProcess(processhandle: t::HANDLE, exitstatus: t::NTSTATUS) -> t::NTSTATUS);
link!("ntdll" fn RtlAllocateHeap(hHeap: t::HANDLE, dwFlags: t::ULONG, dwBytes: usize) -> t::LPVOID);
link!("ntdll" fn RtlFreeHeap(hHeap: t::HANDLE, dwFlags: t::ULONG, lpMem: t::LPVOID) -> t::BOOLEAN);
link!("ntdll" fn RtlReAllocateHeap(hHeap: t::HANDLE, dwFlags: t::ULONG, lpMem: t::LPVOID, size: usize) -> t::LPVOID);
link!("ntdll" fn NtWriteFile(fileHandle: t::HANDLE, event: t::HANDLE, apcRoutine: t::PIO_APC_ROUTINE, apcContext: t::PVOID, ioStatusBlock: *mut t::IO_STATUS_BLOCK, buffer: t::PVOID, length: t::ULONG, byteOffset: t::PLARGE_INTEGER, key: t::PULONG) -> t::NTSTATUS);
link!("ntdll" fn RtlGetCurrentPeb() -> *const peb::PEB);
link!("ntdll" fn NtQueryAttributesFile(ObjectAttributes: *mut t::OBJECT_ATTRIBUTES, FileInformation: *mut t::FILE_BASIC_INFORMATION) -> t::NTSTATUS);
link!("ntdll" fn RtlDosPathNameToNtPathName_U(DosFileName: t::PCWSTR, NtFileName: *mut t::UNICODE_STRING, FilePart: *mut t::PWSTR, RelativeName: t::PVOID) -> t::BOOLEAN);
link!("ntdll" fn RtlFreeUnicodeString(UnicodeString: *mut t::UNICODE_STRING) -> ());
link!("ntdll" fn NtCreateFile(FileHandle: *mut t::HANDLE, DesiredAccess: t::ACCESS_MASK, ObjectAttributes: *mut t::OBJECT_ATTRIBUTES, IoStatusBlock: *mut t::IO_STATUS_BLOCK, AllocationSize: t::PLARGE_INTEGER, FileAttributes: t::ULONG, ShareAccess: t::ULONG, CreateDisposition: t::ULONG, CreateOptions: t::ULONG, EaBuffer: t::PVOID, EaLength: t::ULONG) -> t::NTSTATUS);
link!("ntdll" fn NtClose(handle: t::HANDLE) -> t::NTSTATUS);
