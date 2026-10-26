use crate::{path::Path, raw};

pub type CreateDirError = raw::fs::CreateDirError;

/// # Errors
///
/// Returns `Err` if the directory could not be created.
pub fn create_dir<T: Into<Path>>(path: T) -> Result<(), CreateDirError> {
    create_dir_inner(&path.into())
}

fn create_dir_inner(path: &Path) -> Result<(), CreateDirError> {
    raw::fs::create_dir(path.as_os_str().as_slice())
}

#[cfg(test)]
mod tests {
    use crate::fs;

    #[test]
    fn create_dir_test() {
        let path = "CommonPath";
        fs::create_dir(path).expect("Failed to create dir");

        std::fs::remove_dir(path)
            .expect("STD::FS::REMOVE_DIR FAILED: common::fs::create_dir executed successfully, but failed to remove the directory using std::fs::remove_dir");
    }
}
