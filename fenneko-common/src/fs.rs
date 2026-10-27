use core::fmt;

use winapi::win32::fileapi::FileAttributes;

use crate::{
    path::Path,
    raw::fs::{self, Attributes, AttributesError, OpenError},
};

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

    /// # Errors
    ///
    /// Returns `Err` if the file could not be created.
    pub fn create<T: Into<Path>>(path: T) -> Result<Self, OpenError> {
        fs::create_new_file(path.into().as_os_str().as_slice()).map(Self::from_handle)
    }
}

impl<A: fs::AccessCanWrite> File<A> {
    /// # Errors
    ///
    /// Returns `Err` if it failed to write data to the file.
    #[allow(clippy::needless_pass_by_ref_mut)]
    pub fn write_all(&mut self, data: &[u8]) -> Result<usize, fs::WriteError> {
        fs::write_file(&self.0, data)
    }
}

impl<A: fs::AccessCanWrite> fmt::Write for File<A> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match self.write_all(s.as_bytes()) {
            Ok(_) => Ok(()),
            Err(_) => Err(fmt::Error),
        }
    }
}

/// # Errors
///
/// Returns `Err` if the directory could not be created.
pub fn create_dir<T: Into<Path>>(path: T) -> Result<(), CreateDirError> {
    fs::create_dir(path.into().as_os_str().as_slice(), FileAttributes::new())
}

/// # Errors
///
/// Returns `Err` if the directory could not be created.
pub fn create_dir_with_attrs<T: Into<Path>>(
    path: T,
    attrs: FileAttributes,
) -> Result<(), CreateDirError> {
    fs::create_dir(path.into().as_os_str().as_slice(), attrs)
}

/// # Errors
///
/// Returns `Err` if failed to get file attributes
pub fn metadata<T: Into<Path>>(path: T) -> Result<Attributes, AttributesError> {
    fs::raw_attributes(path.into().as_os_str().as_slice())
}

#[cfg(test)]
mod tests {
    use core::fmt::Write as _;

    use super::*;
    use crate::{
        fs::metadata,
        raw::fs::{AccessRead, AccessWrite},
    };

    #[track_caller]
    fn remove_dir<T: AsRef<std::path::Path>>(path: T) {
        std::fs::remove_dir(path).expect("STD::FS::REMOVE_DIR FAILED");
    }

    #[track_caller]
    fn remove_file<T: AsRef<std::path::Path>>(path: T) {
        std::fs::remove_file(path).expect("STD::FS::REMOVE_FILE FAILED");
    }

    #[test]
    fn create_dir_test() {
        let path = "CommonPath";
        create_dir(path).expect("Failed to create dir");

        remove_dir(path);
    }

    #[test]
    fn write_file_test() {
        let path = "WriteFile";
        std::fs::write(path, "").expect("STD::FS::WRITE FAILED");

        let write_result = {
            let mut file = File::<AccessWrite>::open(path).expect("Failed to open file");

            file.write_all(b"WriteFile: common::fs::tests::write_file_test")
        };

        let data = std::fs::read_to_string(path).expect("STD::FS::READ FAILED");
        remove_file(path);

        write_result.expect("write_all failed");
        assert_eq!(data, "WriteFile: common::fs::tests::write_file_test");
    }

    #[test]
    fn fmt_file_test() {
        let path = "FormatFile";
        std::fs::write(path, "").expect("STD::FS::WRITE FAILED");

        let write_result = {
            let mut file = File::<AccessWrite>::open(path).expect("Failed to open file");
            let fmt = format_args!("{}File: 2 + 2 = {}", "Format", 2 + 2);
            file.write_fmt(fmt)
        };

        let data = std::fs::read_to_string(path).expect("STD::FS::READ FAILED");
        remove_file(path);

        write_result.expect("write_fmt failed");
        assert_eq!(data, "FormatFile: 2 + 2 = 4");
    }

    #[test]
    fn create_file_test() {
        let path = "CreatedFile";

        {
            File::<AccessRead>::create(path).expect("Failed to create file");
        }

        assert!(std::fs::exists(path).expect("Failed to check whether the file was created"));
        remove_file(path);
    }

    #[test]
    fn metadata_test() {
        let _ = metadata("Cargo.toml").expect("Failed to get attrs");

        let attrs = metadata("NotExists");
        assert!(matches!(attrs, Err(AttributesError::FileNotFound)));
    }
}
