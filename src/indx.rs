use alloc::string::String;
use alloc::vec::Vec;

use crate::os::fs::Path;
use crate::os::{error, fs};
use crate::sha1::Sha1;

const INDEX_VERSION: u32 = 2;

#[derive(Debug, Clone)]
pub struct IndexFile {
    pub sha1: [u8; 20],
    pub name: String,
    pub mode: u32,
    pub size: u32,
    pub ctime_sec: u32,
    pub ctime_nsec: u32,
    pub mtime_sec: u32,
    pub mtime_nsec: u32,
    pub dev: u32,
    pub ino: u32,
    pub uid: u32,
    pub gid: u32
}

pub fn read_index() -> error::Result<Vec<IndexFile>> {
    Ok(Vec::new())
}

pub fn write_index(entries: Vec<IndexFile>) -> error::Result<()> {
    // Header
    let mut header: Vec<u8> = Vec::with_capacity(12);
    header.extend(b"DIRC");
    header.extend(INDEX_VERSION.to_be_bytes());
    header.extend((entries.len() as u32).to_be_bytes());

    
    let mut entries = entries;
    entries.sort_by_key(|e| e.name.clone());

    // Body
    let mut body: Vec<u8> = Vec::new();
    for e in &entries {
        let name_len = e.name.len();
        if name_len > 4095 {
            todo!("add flag for >4095 chars in file name")
        }
        let flags = (name_len & 0x0FFF) as u16;

        body.extend(e.ctime_sec.to_be_bytes());
        body.extend(e.ctime_nsec.to_be_bytes());

        body.extend(e.mtime_sec.to_be_bytes());
        body.extend(e.mtime_nsec.to_be_bytes());

        body.extend(e.dev.to_be_bytes());
        body.extend(e.ino.to_be_bytes());
        body.extend(e.mode.to_be_bytes());
        body.extend(e.uid.to_be_bytes());
        body.extend(e.gid.to_be_bytes());
        body.extend(e.size.to_be_bytes());
        body.extend(e.sha1);
        body.extend(flags.to_be_bytes());

        body.extend(e.name.as_bytes());
        let total_len = 62 + name_len;
        let padding = (8 - (total_len % 8)) % 8;
        for _ in 0..padding {
            body.push(0);
        }
    }
    
    // Extensions
    // not implemented

    // Hash
    let mut content = Vec::with_capacity(header.len() + body.len() + 20);
    content.extend(header);
    content.extend(body);
    
    let mut hasher = Sha1::new();
    hasher.encrypt(&content);
    let hash = hasher.bytes();

    content.extend(hash);
    let path = Path::current().join(".git").join("index");
    fs::create_file_all(path, &content[..], content.len());
    
    Ok(())
} 