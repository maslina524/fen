use alloc::string::String;

#[derive(Debug, Clone)]
pub struct IndexFile {
    pub blob_hash: String,
    pub normalized: String,
    pub mode: u32,
    pub size: u64,
    pub ctime_sec: u32,
    pub ctime_nsec: u32,
    pub mtime_sec: u32,
    pub mtime_nsec: u32,
    pub exec_attr: u8,
    pub dev: u8,
    pub ino: u8,
    pub uid: u8,
    pub gid: u8
}