use core::sync::atomic::{AtomicBool, Ordering::Relaxed};

use winapi::win32::{
    console::{CodePage, ConsoleHandle, GetStdHandle, SetConsoleOutputCP, StdHandle},
    fileapi::WriteFile,
};

pub static IS_UTF8: AtomicBool = AtomicBool::new(false);

pub type RawHandle = ConsoleHandle;

/// # Panics
///
/// Panics if it fails to get stdhandle, in theory the panic should never happen
/// since getting stdhandle is always a valid action in a CLI
#[must_use]
pub fn stdout() -> RawHandle {
    GetStdHandle(StdHandle::Stdout).expect("Failed to get std handle (stdout)")
}

/// # Panics
///
/// Panics if it fails to get stdhandle, in theory the panic should never happen
/// since getting stdhandle is always a valid action in a CLI
#[must_use]
pub fn stderr() -> RawHandle {
    GetStdHandle(StdHandle::Stderr).expect("Failed to get std handle (stderr)")
}

/// Returns the number of bytes written.
pub fn write(handle: &RawHandle, bytes: &[u8]) -> Option<usize> {
    if !IS_UTF8.load(Relaxed) {
        IS_UTF8.store(true, Relaxed);
        SetConsoleOutputCP(CodePage::Utf8);
    }
    WriteFile(handle, bytes).ok()
}
