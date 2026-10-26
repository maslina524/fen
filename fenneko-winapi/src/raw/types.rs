use core::{ffi::c_void, mem, ptr};

pub type HANDLE = *mut c_void;
pub type NTSTATUS = i32;
pub type ACCESS_MASK = ULONG;

// Pointers
pub type PULONG = *const u32;
pub type PLARGE_INTEGER = *const i64;
pub type PVOID = *mut c_void;
pub type LPVOID = *mut c_void;
pub type PIO_APC_ROUTINE = *mut c_void;

// Primitives
pub type BYTE = u8;
pub type USHORT = u16;
pub type UINT = u32;
pub type ULONG = u32;
pub type ULONG_PTR = usize;
pub type LARGE_INTEGER = i64;

// Strings
pub type PWSTR = *mut u16;
pub type PCWSTR = *const u16;

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
#[derive(Default)]
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

#[repr(C)]
#[derive(Default)]
pub struct FILE_BASIC_INFORMATION {
    pub CreationTime: LARGE_INTEGER,
    pub LastAccessTime: LARGE_INTEGER,
    pub LastWriteTime: LARGE_INTEGER,
    pub ChangeTime: LARGE_INTEGER,
    pub FileAttributes: ULONG,
}

#[repr(C)]
pub struct OBJECT_ATTRIBUTES {
    pub Length: ULONG,
    pub RootDirectory: HANDLE,
    pub ObjectName: *const UNICODE_STRING,
    pub Attributes: ULONG,
    pub SecurityDescriptor: PVOID,
    pub SecurityQualityOfService: PVOID,
}

impl Default for OBJECT_ATTRIBUTES {
    fn default() -> Self {
        Self {
            Length: mem::size_of::<Self>() as ULONG,
            RootDirectory: ptr::null_mut(),
            ObjectName: ptr::null_mut(),
            Attributes: 0x0000_0040,
            SecurityDescriptor: ptr::null_mut(),
            SecurityQualityOfService: ptr::null_mut(),
        }
    }
}
