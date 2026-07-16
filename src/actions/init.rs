use alloc::string::ToString;

use crate::os::error::ErrorType;
use crate::os::fs;
use crate::{NoResult, toml};

pub fn init() -> NoResult {
    if let Err(e) = fs::create_dir(".git") {
        return match e.typ() {
            ErrorType::DirAlreadyExists => Err("git already initialized in this directory".into()),
            _ => Err("an unexpected error occurred while creating the git directory".into())
        };
    }
    
    fs::set_file_attribute(
        ".git", 
        fs::FILE_ATTRIBUTE_HIDDEN
    )?;

    // Create Dirs
    fs::create_dir(".git/refs")?;              // refs
    fs::create_dir(".git/refs/heads")?;        // refs/heads
    fs::create_dir(".git/refs/tags")?;         // refs/tags
    fs::create_dir(".git/objects")?;           // objects
    fs::create_dir(".git/objects/info")?;      // objects/info
    fs::create_dir(".git/objects/pack")?;      // objects/pack
    fs::create_dir(".git/hooks")?;             // hooks
    fs::create_dir(".git/info")?;              // info

    // // refs/heads/master
    // let content = b"";
    // fs::create_file(".git/refs/heads/master", content, content.len())?;

    // info/exclude
    fs::create_file(".git/info/exclude", b"")?;

    // HEAD
    let content = b"ref: refs/heads/master\n";
    fs::create_file(".git/HEAD", content)?;

    // description
    let content = b"Unnamed repository; edit this file 'description' to name the repository.\n";
    fs::create_file(".git/description", content)?;

    // config
    let map = toml!(
        "core" => {
            "bare" => "false",
            "repositoryformatversion" => "0",
            "filemode" => "false",
            "symlinks" => "false",
            "ignorecase" => "true",
            "logallrefupdates" => "true"
        }
    ).to_string();
    let content = map.as_bytes();
    fs::create_file(".git/config", content)?;

    Ok(())
}