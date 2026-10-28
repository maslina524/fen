use core::{
    error::Error,
    fmt,
    sync::atomic::{AtomicBool, Ordering::Relaxed},
};

use winapi::win32::{
    console::{CodePage, ConsoleHandle, GetStdHandle, SetConsoleOutputCP, StdHandle},
    fileapi::WriteFile,
};

pub static IS_UTF8: AtomicBool = AtomicBool::new(false);

pub type RawHandle = ConsoleHandle;

#[derive(Debug, Clone, Copy)]
pub struct RawStdWriteError;

impl fmt::Display for RawStdWriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for RawStdWriteError {}

/// # Panics
///
/// Panics if it fails to get stdhandle, in theory the panic should never happen
/// since getting stdhandle is always a valid action in a CLI
#[must_use]
pub fn stdout() -> RawHandle {
    if let Ok(h) = GetStdHandle(StdHandle::Stdout) {
        return h;
    }
    panic!("Failed to get std handle (stdout)");
}

/// # Panics
///
/// Panics if it fails to get stdhandle, in theory the panic should never happen
/// since getting stdhandle is always a valid action in a CLI
#[must_use]
pub fn stderr() -> RawHandle {
    if let Ok(h) = GetStdHandle(StdHandle::Stderr) {
        return h;
    }
    panic!("Failed to get std handle (stderr)");
}

/// # Errors
///
/// Returns `Err` when it fails to write bytes to Raw Std handle.
pub fn write(handle: &RawHandle, bytes: &[u8]) -> Result<usize, RawStdWriteError> {
    if !IS_UTF8.load(Relaxed) {
        IS_UTF8.store(true, Relaxed);
        SetConsoleOutputCP(CodePage::Utf8);
    }
    WriteFile(handle, bytes).map_err(|_| RawStdWriteError)
}
