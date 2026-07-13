use alloc::string::String;
use alloc::vec::Vec;

use crate::{NoResult, blob, glob, indx, println};
use crate::os::fs::{self, Path};
use crate::indx::IndexFile;

pub fn add(patterns: &[String]) -> NoResult {
    if patterns.is_empty() {
        return Err("No file patterns to add to index".into());
    }

    let files = find_files(patterns);
    let mut index = indx::read_index()?;
    for file in files {
        let sha1 = blob::write_blob(&file)?;
        let name = file.normalize_string();
        let mode = 0o100644; // only for windows

        let info = fs::get_file_info(&file)?;
        let size = info.size as u32;
        let ctime_sec = info.ctime_sec;
        let ctime_nsec = info.ctime_nsec;
        let mtime_sec = info.mtime_sec;
        let mtime_nsec = info.mtime_nsec;

        let dev = 0; // only for windows
        let ino = 0; // only for windows
        let uid = 0; // only for windows
        let gid = 0; // only for windows

        index.push( IndexFile {
            sha1, name, mode, size, ctime_sec, ctime_nsec,
            mtime_sec, mtime_nsec,  dev, ino, uid, gid
        } );
    }

    indx::write_index(index)?;

    Ok(())
}

fn find_files(patterns: &[String]) -> Vec<Path> {
    let mut results = Vec::new();
    for pat in patterns {
        let pat = pat.replace('\\', "/");
        let parts: Vec<&str> = pat.split('/').collect();
        let root = Path::current();
        search_recursive(&root, &parts, &mut results);
    }
    results
}

fn search_recursive(current: &Path, parts: &[&str], results: &mut Vec<Path>) {
    if parts.is_empty() {
        if current.is_file() {
            results.push(current.clone());
        }
        return;
    }

    let part = parts[0];
    let rest = &parts[1..];

    if part == "**" {
        search_recursive(current, rest, results);

        if let Ok(items) = fs::read_dir(current.clone()) {
            for item in items {
                let name = item.name();
                let sub_path = current.clone().join(&name);
                if sub_path.is_dir() {
                    search_recursive(&sub_path, parts, results);
                }
            }
        }
        return;
    }

    let items = match fs::read_dir(current.clone()) {
        Ok(v) => v,
        Err(_) => return,
    };

    for item in items {
        let name = item.name();
        if glob::glob(part, &name) {
            let sub_path = current.clone().join(&name);

            if rest.is_empty() {
                if sub_path.is_file() {
                    results.push(sub_path);
                }
            } else {
                if sub_path.is_dir() {
                    search_recursive(&sub_path, rest, results);
                }
            }
        }
    }
}