use core::marker::PhantomData;

use crate::{Handle, raw::HANDLE, win32::handleapi::CloseHandle};

import!(CreateFile, GetFileAttributes, WriteFile);

mod_sealed!();
pub trait FileAccess: sealed::Sealed {
    fn raw() -> u32;
}

pub trait CanRead: FileAccess {}
pub trait CanWrite: FileAccess {}

pub struct Read;
pub struct Write;
pub struct All;

impl_tag!(CanRead: Read, All);
impl_tag!(CanWrite: Write, All);

impl_sealed!(Read, Write, All);

impl FileAccess for Read {
    fn raw() -> u32 {
        0x8000_0000
    }
}

impl FileAccess for Write {
    fn raw() -> u32 {
        0x4000_0000
    }
}

impl FileAccess for All {
    fn raw() -> u32 {
        0x8000_0000 | 0x4000_0000
    }
}

#[repr(transparent)]
pub struct FileHandle<Access> {
    raw: HANDLE,
    _marker: PhantomData<Access>,
}

impl<Access> Handle for FileHandle<Access> {
    fn from_raw(raw: HANDLE) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    fn as_raw(&self) -> HANDLE {
        self.raw
    }
}

impl<Access> Drop for FileHandle<Access> {
    fn drop(&mut self) {
        CloseHandle(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::win32::{
        console::{GetStdHandle, StdHandle},
        fileapi::GetFileAttributes::{ItemAttributes, ItemType},
    };

    #[test]
    fn write_to_stdout_test() {
        let handle = GetStdHandle(StdHandle::Stdout).expect("Failed to get std handle");
        let string = "Hello World!\n";
        let written =
            WriteFile(&handle, string.as_bytes()).expect("Failed to write to the console");
        assert_eq!(written, string.len());
    }

    #[test]
    fn get_file_attrs_test() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), r"\Cargo.toml");
        println!("{path:?}");
        let encoded: Vec<u16> = path.encode_utf16().collect();

        let attrs = GetFileAttributes(&encoded).expect("Failed to get file attrs");
        assert_eq!(
            attrs,
            ItemAttributes {
                is_readonly: false,
                is_hidden: false,
                is_system: false,
                typ: ItemType::File
            }
        );
    }

    #[test]
    fn create_file_test() {
        let path: Vec<u16> = "Cargo.toml".encode_utf16().collect();
        CreateFile::<Write>(
            &path,
            FileShare::ReadWrite,
            FileDispose::OpenAlways,
            FileFlags::new().normal(),
        )
        .expect("Failed to open file");
    }
}
