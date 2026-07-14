use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::string::{ToString, String};
use alloc::format;

use crate::os::env;
use crate::os::fs::{self, Path, create_file};
use crate::os::error;
use crate::sha1::Sha1;
use crate::zlib;

const NAME: &str = "temp_name";
const EMAIL: &str = "example@gmail.com";

pub fn get_head() -> Result<Option<[u8; 40]>, Box<dyn core::error::Error>> {
    let head_path_raw = fs::read_to_string(".git/HEAD")?;
    if !head_path_raw.starts_with("ref: ") {
        return Err("HEAD file corrupted".into());
    }

    let head_path = format!(".git/{}", &head_path_raw[5..]);
    let head_bytes = fs::read_to_bytes(head_path.trim())?;
    
    if head_bytes.len() == 0 {
        return Ok(None);
    }

    let ret = head_bytes.try_into().map_err(|_| "HEAD hash corrupted")?;
    Ok(Some(ret))
}

pub fn set_head(hash: &Sha1) -> Result<(), Box<dyn core::error::Error>> {
    let head_path_raw = fs::read_to_string(".git/HEAD")?;
    if !head_path_raw.starts_with("ref: ") {
        return Err("HEAD file corrupted".into());
    }

    let head_path = format!(".git/{}", &head_path_raw[5..]);
    let hex_hash = hash.hex();
    let content = hex_hash.as_bytes();
    fs::create_file_all(head_path.trim(), content, content.len())?;

    Ok(())
}

pub fn write_commit(tree: &Sha1, msg: &str) -> Result<Sha1, Box<dyn core::error::Error>> {
    let mut body = Vec::new();

    // Tree
    body.extend(b"tree ");
    body.extend(tree.hex().as_bytes());
    body.push(0xA);

    // Parent
    if let Some(hash) = get_head()? {
        body.extend(b"parent ");
        body.extend(hash);
        body.push(0xA);
    };

    // Author
    let timestamp = env::timestamp();
    let tz = env::get_time_zone_string();
    body.extend(format!("author {NAME} <{EMAIL}> {timestamp} {tz}\n").as_bytes());

    // Commiter
    let timestamp = env::timestamp();
    let tz = env::get_time_zone_string();
    body.extend(format!("commiter {NAME} <{EMAIL}> {timestamp} {tz}\n\n").as_bytes());

    // Message
    body.extend(format!("{msg}\n").as_bytes());

    let mut raw = Vec::with_capacity(body.len() + 12);
    raw.extend(b"commit ");
    raw.extend(body.len().to_string().as_bytes());
    raw.push(0x00);
    raw.extend(body);

    let mut hasher = Sha1::new();
    hasher.encrypt(&raw);
    let hash = hasher.hex();

    let mut buf = Vec::new();
    zlib::compress(&raw, &mut buf);

    let save_path = Path::current().join(".git").join("objects").join(&hash[..2]).join(&hash[2..]);
    fs::create_file_all(&save_path, &buf[..], buf.len())?;

    crate::println!("commit hash: {}", hash);
    set_head(&hasher)?;

    Ok( hasher )
} 