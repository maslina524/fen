use winapi::api::{
    console::{GetStdHandle, StdHandle},
    file::{FileHandle, Write, WriteFile},
};

pub type RawHandle = FileHandle<Write>;

pub fn stdout() -> RawHandle {
    GetStdHandle(StdHandle::Stdout).expect("Failed to get std handle (stdout)")
}

pub fn stderr() -> RawHandle {
    GetStdHandle(StdHandle::Stderr).expect("Failed to get std handle (stderr)")
}

/// Returns the number of bytes written.
pub fn write(handle: &RawHandle, bytes: &[u8]) -> Option<usize> {
    WriteFile(handle, bytes).ok()
}
