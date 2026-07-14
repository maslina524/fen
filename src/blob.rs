use alloc::vec::Vec;
use alloc::string::String;
use alloc::format;

use crate::os::error;
use crate::os::fs::{self, Path};
use crate::sha1::Sha1;
use crate::zlib;
use crate::println;

pub fn write_blob<T: Into<Path>>(path: T) -> error::Result<Sha1> {
    let path = path.into();
    let mut raw_buf = Vec::new();
    let file_content = fs::read_to_bytes(&path)?;
    let header = format!("blob {}", file_content.len());

    raw_buf.extend(header.as_bytes());
    raw_buf.push(0);
    raw_buf.extend(file_content);

    let mut hasher = Sha1::new();
    hasher.encrypt(&raw_buf);
    let hash = hasher.hex();

    let mut buf = Vec::new();
    zlib::compress(&raw_buf, &mut buf);

    let save_path = Path::current().join(".git").join("objects").join(&hash[..2]).join(&hash[2..]);
    fs::create_file_all(save_path, &buf[..], buf.len())?;
    
    Ok( hasher )
}