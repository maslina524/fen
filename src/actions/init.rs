use alloc::format;
use alloc::string::ToString;

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

    // HEAD
    let content = b"ref: refs/heads/master\n";
    if let Err(e) = fs::create_file(".git/HEAD", content, content.len()) {
        return Err(format!("{}", e.to_string()).into());
    };

    // description
    let content = b"Unnamed repository; edit this file 'description' to name the repository.\n";
    if let Err(e) = fs::create_file(".git/description", content, content.len()) {
        return Err(format!("{}", e.to_string()).into());
    };

    Ok(())
}