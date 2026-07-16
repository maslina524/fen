use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::boxed::Box;

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

pub fn read_index() -> Result<Vec<IndexFile>, Box<dyn core::error::Error>> {
    let path = Path::current().join(".git").join("index");
    if !fs::exists(&path) {
        return Ok(Vec::new());
    }

    let data = fs::read_to_bytes(&path)?;
    if data.len() < 12 {
        return Err("Index file too small".into());
    }

    let sig = &data[0..4];
    if sig != b"DIRC" {
        return Err("Invalid index signature".into());
    }

    let version = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
    if version != INDEX_VERSION {
        return Err("Unsupported index version".into());
    }

    let count = u32::from_be_bytes([data[8], data[9], data[10], data[11]]) as usize;

    let mut entries = Vec::with_capacity(count);
    let mut pos = 12;

    for _ in 0..count {
        if pos + 62 + 1 + 1 + 20 > data.len() {
            return Err("index truncated".into());
        }

        let ctime_sec = u32::from_be_bytes(data[pos..pos+4].try_into().unwrap());
        pos += 4;
        let ctime_nsec = u32::from_be_bytes(data[pos..pos+4].try_into().unwrap());
        pos += 4;
        let mtime_sec = u32::from_be_bytes(data[pos..pos+4].try_into().unwrap());
        pos += 4;
        let mtime_nsec = u32::from_be_bytes(data[pos..pos+4].try_into().unwrap());
        pos += 4;
        let dev = u32::from_be_bytes(data[pos..pos+4].try_into().unwrap());
        pos += 4;
        let ino = u32::from_be_bytes(data[pos..pos+4].try_into().unwrap());
        pos += 4;
        let mode = u32::from_be_bytes(data[pos..pos+4].try_into().unwrap());
        pos += 4;
        let uid = u32::from_be_bytes(data[pos..pos+4].try_into().unwrap());
        pos += 4;
        let gid = u32::from_be_bytes(data[pos..pos+4].try_into().unwrap());
        pos += 4;
        let size = u32::from_be_bytes(data[pos..pos+4].try_into().unwrap());
        pos += 4;
        let mut sha1 = [0u8; 20];
        sha1.copy_from_slice(&data[pos..pos+20]);
        pos += 20;
        let flags = u16::from_be_bytes(data[pos..pos+2].try_into().unwrap());
        pos += 2;

        let name_len = (flags & 0x0FFF) as usize;

        if pos + name_len + 1 > data.len() - 20 {
            return Err("index entry name too long or missing terminator".into());
        }

        let name_bytes = &data[pos..pos + name_len];
        let name = match core::str::from_utf8(name_bytes) {
            Ok(s) => s.to_string(),
            Err(_) => return Err("invalid UTF-8 in index entry name".into()),
        };
        pos += name_len;

        let term = data[pos];
        if term != 0 {
            return Err("missing null terminator after index entry name".into());
        }
        pos += 1;

        let total_len = 62 + name_len + 1;
        let padding = (8 - (total_len % 8)) % 8;
        pos += padding;

        entries.push( IndexFile {
            sha1, name, mode, size, ctime_sec, ctime_nsec,
            mtime_sec, mtime_nsec, dev, ino, uid, gid,
        } );
    }

    if data.len() < 20 {
        return Err("index missing checksum".into());
    }
    let checksum_start = data.len() - 20;
    let actual = &data[checksum_start..];

    let mut hasher = Sha1::new();
    hasher.encrypt(&data[..checksum_start]);
    let expected = hasher.bytes();

    if actual != expected {
        return Err("index checksum mismatch".into());
    }

    Ok(entries)
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
        body.push(0);
        let total_len = 62 + name_len + 1;
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
    fs::create_file_all(path, &content[..]);
    
    Ok(())
} 