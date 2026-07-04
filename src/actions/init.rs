use crate::NoResult;
use crate::os::error::ErrorType;
use crate::os::fs;

pub fn init() -> NoResult {
    if let Err(e) = fs::create_dir(".git", true) {
        return match e.typ() {
            ErrorType::DirAlreadyExists => Err("git already initialized in this directory".into()),
            _ => Err("an unexpected error occurred while creating the git directory".into())
        };
    }

    // Create Dirs
    fs::create_dir("ref", false)?;              // refs
    fs::create_dir("ref/heads", false)?;        // refs/heads
    fs::create_dir("ref/tags", false)?;         // refs/tags
    fs::create_dir("objects", false)?;          // objects
    fs::create_dir("objects/info", false)?;     // objects/info
    fs::create_dir("objects/pack", false)?;     // objects/pack
    fs::create_dir("hooks", false)?;            // hooks
    fs::create_dir("info", false)?;             // info

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