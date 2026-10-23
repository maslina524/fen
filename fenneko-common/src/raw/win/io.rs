use core::sync::atomic::{AtomicBool, Ordering::Relaxed};

use winapi::api::{
    console::{CodePage, GetStdHandle, SetConsoleOutputCP, StdHandle},
    file::{FileHandle, Write, WriteFile},
};

pub static IS_UTF8: AtomicBool = AtomicBool::new(false);

pub type RawHandle = FileHandle<Write>;

#[must_use]
pub fn stdout() -> RawHandle {
    GetStdHandle(StdHandle::Stdout).expect("Failed to get std handle (stdout)")
}

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
