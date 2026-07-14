use alloc::boxed::Box;
use alloc::borrow::ToOwned;
use alloc::vec::Vec;
use alloc::string::{ToString, String};
use alloc::format;

use crate::indx::{self, IndexFile};
use crate::os::fs::{self, Path};
use crate::zlib;
use crate::sha1::Sha1;

struct TreeEntry {
    mode: u32,
    name: String,
    hash: [u8; 20],
}

pub fn write_tree(entries: &Vec<IndexFile>, prefix: &str) -> Result<Sha1, Box<dyn core::error::Error>> {
    let mut file_entries = Vec::new();
    let mut dir_prefixes = Vec::new();
    let prefix_with_slash = if prefix.is_empty() { String::new() } else { format!("{}/", prefix) };

    for e in entries {
        if !e.name.starts_with(&prefix_with_slash) {
            continue;
        }
        let rel = &e.name[prefix_with_slash.len()..];
        if rel.is_empty() {
            continue;
        }
        if let Some(next_slash) = rel.find('/') {
            let dir_name = &rel[..next_slash];
            if !dir_prefixes.contains(&dir_name) {
                dir_prefixes.push(dir_name);
            }
        } else {
            file_entries.push(e);
        }
    }

    let mut tree_entries = Vec::new();
    for dir_name in dir_prefixes {
        let sub_prefix = if prefix.is_empty() {
            dir_name
        } else {
            &format!("{}/{}", prefix, dir_name)
        };
        let hash = write_tree(entries, &sub_prefix)?;
        tree_entries.push(TreeEntry {
            mode: 0o40000,
            name: dir_name.to_owned(),
            hash: hash.bytes(),
        });
    }

    for e in file_entries {
        let name = e.name.rsplit('/').next().unwrap().to_string();
        tree_entries.push(TreeEntry {
            mode: e.mode,
            name,
            hash: e.sha1,
        });
    }

    tree_entries.sort_by(|a, b| a.name.cmp(&b.name));

    let mut data = Vec::new();
    for te in &tree_entries {
        let mode_str = format!("{:o}", te.mode);
        data.extend(mode_str.as_bytes());
        data.push(0x20);
        data.extend(te.name.as_bytes());
        data.push(0x00);
        data.extend(&te.hash);
    }

    let header = format!("tree {}\0", data.len());
    let mut raw = Vec::with_capacity(header.len() + data.len());
    raw.extend(header.as_bytes());
    raw.extend(data);

    let mut hasher = Sha1::new();
    hasher.encrypt(&raw);
    let hash = hasher.hex();

    let mut buf = Vec::new();
    zlib::compress(&raw, &mut buf);

    let save_path = Path::current().join(".git").join("objects").join(&hash[..2]).join(&hash[2..]);
    fs::create_file_all(save_path, &buf[..], buf.len())?;

    crate::println!("tree hash: {}", hash);

    Ok( hasher )
}