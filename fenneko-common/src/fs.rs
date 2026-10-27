use crate::{path::Path, raw::fs::{self, OpenError}};

pub type CreateDirError = fs::CreateError;
pub struct File<A: fs::Access>(fs::RawHandle<A>);

impl<A: fs::Access> File<A> {
    const fn from_handle(handle: fs::RawHandle<A>) -> Self {
        Self(handle)
    }

    /// # Errors
    ///
    /// Returns `Err` if the file could not be opened.
    pub fn open<T: Into<Path>>(path: T) -> Result<Self, OpenError> {
        fs::open_file(path.into().as_os_str().as_slice()).map(Self::from_handle)
    }
}

impl<A: fs::AccessCanWrite> File<A> {
    /// # Errors
     ///
     /// Returns `Err` if it failed to write data to the file.
    pub fn write(&self, data: &[u8]) -> Result<usize, fs::WriteError> {
        fs::write_file(&self.0, data)
    }
}

/// # Errors
///
/// Returns `Err` if the directory could not be created.
pub fn create_dir<T: Into<Path>>(path: T) -> Result<(), CreateDirError> {
    fs::create_dir(path.into().as_os_str().as_slice())
}

#[cfg(test)]
mod tests {
    use crate::raw::fs::AccessWrite;

    use super::*;

    #[test]
    fn create_dir_test() {
        let path = "CommonPath";
        create_dir(path).expect("Failed to create dir");

        std::fs::remove_dir(path)
            .expect("STD::FS::REMOVE_DIR FAILED: common::fs::create_dir executed successfully, but failed to remove the directory using std::fs::remove_dir");
    }

    #[test]
    fn write_file_test() {
        let path = "WriteFile";
        std::fs::write(path, "").expect("STD::FS::WRITE FAILED");
        
        {
            let file = File::<AccessWrite>::open(path).expect("Failed to open file");
            file.write(b"WriteFile: common::fs::tests::write_file_test").expect("Failed to write to the file");
        }    

        let data = std::fs::read_to_string(path).expect("STD::FS::READ_TO_STRING FAILED");
        assert_eq!(data, "WriteFile: common::fs::tests::write_file_test");

        std::fs::remove_file(path)
            .expect("STD::FS::REMOVE_FILE FAILED: common::fs::File::write executed successfully, but failed to remove the file using std::fs::remove_file");
    }
}
