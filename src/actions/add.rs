use alloc::collections::BTreeMap;
use alloc::string::{ToString, String};
use alloc::vec::Vec;
use alloc::vec;

use crate::consts::*;
use crate::{FenResult, blob, glob, indx, println};
use crate::os::fs::{self, Path};
use crate::indx::IndexFile;

pub fn add(patterns: &[String], map: BTreeMap<String, String>) -> FenResult<()> {
    if patterns.is_empty() {
        return Err("No file patterns to add to index".into());
    }

    let gitignore = vec![".git/*"];
    let files = find_files(patterns, &gitignore);
    let mut index = indx::read_index()?;
    for file in &files {
        let sha1 = blob::write_blob(file)?.bytes();
        let name = file.normalize_string();
        let mode = 0o100644; // only for windows

        let info = fs::get_file_info(file)?;
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

    if map.contains_key("show") {
        for file in &files {
            let string = file.normalize_string();
            let extension = {
                let ext_str = if let Some(dot) = string.rfind(".") {
                    &string[dot + 1..]
                } else {
                    ""
                };

                if let Some(ext) = extension_colored_icons().get(ext_str) {
                    ext
                } else {
                    FILE_ICON
                }
            };
            println!("{extension} {string}\x1b[0m");
        }
        println!();
    }

    println!("Added {} files", files.len());

    Ok(())
}

fn find_files(patterns: &[String], gitignore: &Vec<&str>) -> Vec<Path> {
    let mut results = Vec::new();
    for pat in patterns {
        let pat = pat.replace('\\', "/");
        let parts: Vec<&str> = pat.split('/').collect();
        let root = Path::current();
        search_recursive(&root, &parts, gitignore, &mut results);
    }
    results
}

fn search_recursive(current: &Path, parts: &[&str], gitignore: &Vec<&str>, results: &mut Vec<Path>) {
    if parts.is_empty() {
        if current.is_file() {
            push(current.clone(), results, gitignore);
        }
        return;
    }

    let part = parts[0];
    let rest = &parts[1..];

    if part == "**" {
        search_recursive(current, rest, gitignore, results);

        if let Ok(items) = fs::read_dir(current.clone()) {
            for item in items {
                let name = item.name();
                let sub_path = current.clone().join(&name);
                if sub_path.is_dir() {
                    search_recursive(&sub_path, parts, gitignore, results);
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
        if glob::glob(&name, part) {
            let sub_path = current.clone().join(&name);

            if rest.is_empty() {
                if sub_path.is_file() {
                    push(sub_path, results, gitignore);
                }
            } else {
                if sub_path.is_dir() {
                    search_recursive(&sub_path, rest, gitignore, results);
                }
            }
        }
    }
}

fn push(path: Path, results: &mut Vec<Path>, gitignore: &Vec<&str>) {
    for ignore in gitignore {
        if glob::glob(&path.normalize_string(), ignore) {
            return;
        }
    }
    results.push(path);
}