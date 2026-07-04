use crate::NoResult;
use crate::os::error::ErrorType;
use crate::os::fs::{self, *};

pub fn init() -> NoResult {
    if let Err(e) = fs::create_dir(".git") {
        return match e.typ() {
            ErrorType::DirAlreadyExists => Err("git already initialized in this directory".into()),
            _ => Err("an unexpected error occurred while creating the git directory".into())
        };
    }
    fs::set_file_attribute(
        ".git", 
        FILE_ATTRIBUTE_HIDDEN | FILE_ATTRIBUTE_READONLY
    );

    // Create Dirs
    fs::create_dir("ref")?;              // refs
    fs::create_dir("ref/heads")?;        // refs/heads
    fs::create_dir("ref/tags")?;         // refs/tags
    fs::create_dir("objects")?;          // objects
    fs::create_dir("objects/info")?;     // objects/info
    fs::create_dir("objects/pack")?;     // objects/pack
    fs::create_dir("hooks")?;            // hooks
    fs::create_dir("info")?;             // info

    // info/exclude
    fs::create_file(".git/info/exclude", b"", 0)?;

    // HEAD
    let content = b"ref: refs/heads/master\n";
    fs::create_file(".git/HEAD", content, content.len())?;

    // description
    let content = b"Unnamed repository; edit this file 'description' to name the repository.\n";
    fs::create_file(".git/description", content, content.len())?;

    Ok(())
}