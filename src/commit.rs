use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::format;

use crate::os::fs;
use crate::os::error;

pub fn get_head() -> Result<[u8; 40], Box<dyn core::error::Error>> {
    let head_path_bytes = fs::read_to_bytes(".git/HEAD")?;
    let head_path_raw = String::from_utf8(head_path_bytes)?;
    if !head_path_raw.starts_with("ref: ") {
        return Err("HEAD file corrupted".into());
    }

    let head_path = format!(".git/{}", &head_path_raw[5..]);
    let head_bytes: [u8; 40] = fs::read_to_bytes(head_path)?
        .try_into()
        .map_err(|_| "HEAD hash corrupted")?;

    Ok( head_bytes )
}