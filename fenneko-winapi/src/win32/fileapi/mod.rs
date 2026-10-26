use core::marker::PhantomData;

use crate::{Handle, raw::HANDLE};

import!(WriteFile, GetFileAttributes);

pub mod FileTag {
    pub trait CanRead {}
    pub trait CanWrite {}

    pub struct Read;
    pub struct Write;
    pub struct All;

    impl CanRead for Read {}
    impl CanRead for All {}
    impl CanWrite for Write {}
    impl CanWrite for All {}
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct FileHandle<Tag> {
    raw: HANDLE,
    _marker: PhantomData<Tag>,
}

impl<Tag> Handle for FileHandle<Tag> {
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
}
