use core::{marker::PhantomData, ptr};

use crate::{
    Handle, NT_SUCCESS,
    raw::{FILE_BASIC_INFORMATION, HANDLE, IO_STATUS_BLOCK, NTSTATUS, OBJECT_ATTRIBUTES, ntdll},
    unicode::UnicodeString,
};

pub trait CanRead {}
pub trait CanWrite {}

pub struct Read;
pub struct Write;
pub struct All;

impl CanRead for Read {}
impl CanRead for All {}
impl CanWrite for Write {}
impl CanWrite for All {}

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

/// # Errors
///
/// Mostly it always returns ``Ok()`` with the number of bytes written
/// in it; situations where ``Err()`` is returned are kernel failures,
/// for example the disk is full.
pub fn WriteFile<T: CanWrite>(handle: &FileHandle<T>, buf: &[u8]) -> Result<usize, NTSTATUS> {
    let mut info = IO_STATUS_BLOCK::default();
    // SAFETY: The descriptor, buffer, and its length are always valid
    let status = unsafe {
        ntdll::NtWriteFile(
            handle.as_raw(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            &raw mut info,
            buf.as_ptr().cast_mut().cast(),
            buf.len() as u32,
            ptr::null(),
            ptr::null(),
        )
    };

    if NT_SUCCESS!(status) {
        let written = info.Information;
        Ok(written)
    } else {
        Err(status)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ItemType {
    Dir,
    File,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemAttributes {
    pub is_readonly: bool,
    pub is_hidden: bool,
    pub is_system: bool,
    pub typ: ItemType,
}

impl ItemAttributes {
    pub const DIRECTORY: u32 = 0b10000;
    pub const HIDDEN: u32 = 0b00010;
    pub const READONLY: u32 = 0b00001;
    pub const SYSTEM: u32 = 0b00100;
}

#[derive(Debug, Clone, Copy)]
pub enum FileAttributesError {
    UnicodePathAllocate,
    QueryAttributes(NTSTATUS),
}

pub fn GetFileAttributes(path: &[u16]) -> Result<ItemAttributes, FileAttributesError> {
    let path = UnicodeString::from_path(path).ok_or(FileAttributesError::UnicodePathAllocate)?;
    let mut attr = OBJECT_ATTRIBUTES {
        ObjectName: path.as_inner_ptr(),
        ..Default::default()
    };
    let mut info = FILE_BASIC_INFORMATION::default();

    let ret = unsafe { ntdll::NtQueryAttributesFile(&raw mut attr, &raw mut info) };
    if !NT_SUCCESS!(ret) {
        return Err(FileAttributesError::QueryAttributes(ret));
    }

    let raw = info.FileAttributes;
    let has = |flag: u32| raw & flag != 0;

    Ok(ItemAttributes {
        is_readonly: has(ItemAttributes::READONLY),
        is_hidden: has(ItemAttributes::HIDDEN),
        is_system: has(ItemAttributes::SYSTEM),
        typ: if has(ItemAttributes::DIRECTORY) {
            ItemType::Dir
        } else {
            ItemType::File
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::console::{GetStdHandle, StdHandle};

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
        let encoded: Vec<u16> = path
            .encode_utf16()
            .collect();
        
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
