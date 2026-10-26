use crate::{
    raw::{FILE_BASIC_INFORMATION, NTSTATUS, OBJECT_ATTRIBUTES, ntdll},
    unicode::UnicodeString,
};

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
