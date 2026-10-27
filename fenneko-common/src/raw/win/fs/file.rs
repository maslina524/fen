use core::{error::Error, fmt};

pub use winapi::win32::fileapi::{
    CanRead as AccessCanRead, CanWrite as AccessCanWrite, FileAccess as Access,
};
use winapi::{
    ntstatus::Ntstatus, win32::fileapi::{
        self, All, Append, CanWrite, CreateFile, CreateOptions, FileAttributes, FileDispose, FileHandle, FileShare, Read, Write, WriteFile,
    },
};

pub type RawHandle<T> = FileHandle<T>;
pub type AccessRead = Read;
pub type AccessWrite = Write;
pub type AccessAppend = Append;
pub type AccessAll = All;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpenError {
    PathNotFound,
    NameNotFound,
    AccessDenied,
    AccessPending,
    InvalidParam,
    InvalidName,
    DiskFull,
    AlreadyExists,
    NameTooLong,
    NotADir,
    QuotaExceeded,
    NoSystemResource,
    BadPrivilege,
    Unknown(Ntstatus),
}

impl fmt::Display for OpenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for OpenError {}

/// # Errors
///
/// Returns `Err` if the file could not be opened.
///
/// # Panics
///
/// Panics if the path could not be converted to Unicode, this means:
/// - `Path`/`OsStr` doesn't work correctly.
/// - An invalid `&[u16]` is passed to the function.
#[track_caller]
fn open_template<A: Access>(path: &[u16], dispose: FileDispose) -> Result<RawHandle<A>, OpenError> {
    const fn map_ntstatus(nt: Ntstatus) -> OpenError {
        use crate::raw::fs::OpenError as OE;

        #[allow(clippy::match_single_binding)]
        match nt {
            _ => OE::Unknown(nt),
        }
    }

    let ret = CreateFile(
        path,
        FileShare::Read,
        dispose,
        FileAttributes::new(),
        CreateOptions::new().non_directory().synchronize(),
    );
    let handle = ret.map_err(|e| match e {
        fileapi::CreateFileError::UnicodePathAllocate => panic!("Bad path"),
        fileapi::CreateFileError::FailedToCreate(nt) => map_ntstatus(nt),
    })?;

    Ok(handle)
}

/// # Errors
///
/// Returns `Err` if the file could not be opened.
pub fn open_file<A: Access>(path: &[u16]) -> Result<RawHandle<A>, OpenError> {
    open_template(path, FileDispose::OpenExisting)
}

/// # Errors
///
/// Returns `Err` if the file could not be opened.
pub fn create_always_file<A: Access>(path: &[u16]) -> Result<RawHandle<A>, OpenError> {
    open_template(path, FileDispose::CreateAlways)
}

/// # Errors
///
/// Returns `Err` if the file could not be opened.
pub fn create_new_file<A: Access>(path: &[u16]) -> Result<RawHandle<A>, OpenError> {
    open_template(path, FileDispose::CreateNew)
}

/// # Errors
///
/// Returns `Err` if the file could not be opened.
pub fn create_file_if_not_exists<A: Access>(path: &[u16]) -> Result<RawHandle<A>, OpenError> {
    open_template(path, FileDispose::OpenAlways)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteError {
    InvalidHandle,
    NotEnoughMemory,
    WriteProtect,
    NotReady,
    Crc,
    WriteFault,
    AccessDenied,
    InvalidParam,
    NoData,
    DiskFull,
    Unknonw(Ntstatus),
}

impl fmt::Display for WriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for WriteError {}

/// # Errors
///
/// Returns `Err` if it failed to write data to the file.
///
/// # Panics
///
/// Panics if the path could not be converted to Unicode, this means:
/// - `Path`/`OsStr` doesn't work correctly.
/// - An invalid `&[u16]` is passed to the function.
pub fn write_file<A: CanWrite>(handle: &FileHandle<A>, buf: &[u8]) -> Result<usize, WriteError> {
    const fn map_ntstatus(nt: Ntstatus) -> WriteError {
        use crate::raw::fs::WriteError as WE;

        #[allow(clippy::match_single_binding)]
        match nt.into_inner() {
            0xC000_0008 => WE::InvalidHandle,
            0xC000_009A | 0xC000_0017 => WE::NotEnoughMemory,
            0xC000_00A2 => WE::WriteProtect,
            0xC000_026E | 0xC000_00A3 => WE::NotReady,
            0xC000_003F => WE::Crc,
            0xC000_009C => WE::WriteFault,
            0xC000_0022 => WE::AccessDenied,
            0xC000_000D => WE::InvalidParam,
            0xC000_00F2 | 0xC000_00AE => WE::NoData,
            0xC000_007F => WE::DiskFull,
            _ => WE::Unknonw(nt),
        }
    }

    WriteFile(handle, buf).map_err(map_ntstatus)
}
