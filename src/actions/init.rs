use crate::NoResult;
use crate::os::fs::{self, FsError};

pub fn init() -> NoResult {
    if let Err(e) = fs::create_dir(".git") {
        return match e {
            FsError::DirAlreadyExists => Err("git already initialized in this directory".into()),
            _ => Err("an unexpected error occurred while creating the git directory".into())
        };
    }

    Ok(())
}