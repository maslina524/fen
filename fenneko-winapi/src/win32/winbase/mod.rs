pub mod CreateDirectory;
pub use CreateDirectory::*;

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::win32::fileapi::FileFlags;

    const FILE_ATTRIBUTE_HIDDEN: u32 = 1 << 1;

    #[test]
    fn create_directory_test() {
        let path: Vec<u16> = "NewDirectory".encode_utf16().collect();
        CreateDirectory(&path, FileFlags::new()).expect("Failed to create dir");

        let ret = fs::exists("NewDirectory").expect("fs::exists failed");
        assert!(ret);

        fs::remove_dir("NewDirectory")
            .expect("FS::REMOVE_DIR FAILED: CreateDirectory executed successfully, but failed to remove the directory using std::fs::remove_dir");
    }

    #[test]
    fn create_hidden_test() {
        unsafe extern "system" {
            safe fn GetFileAttributesA(lpFileName: *const u8) -> u32;
        }

        let path: Vec<u16> = "HiddenDirectory".encode_utf16().collect();
        CreateDirectory(&path, FileFlags::new().hidden()).expect("Failed to create dir");

        let ret = fs::exists("HiddenDirectory").expect("fs::exists failed");
        assert!(ret);

        let meta = GetFileAttributesA(c"HiddenDirectory".as_ptr().cast());
        assert_ne!(
            meta,
            u32::MAX,
            "Failed to get attributes via `GetFileAttributesA`"
        );
        assert!(meta & FILE_ATTRIBUTE_HIDDEN != 0, "File is not hidden");

        fs::remove_dir("HiddenDirectory")
            .expect("FS::REMOVE_DIR FAILED: CreateDirectory executed successfully, but failed to remove the directory using std::fs::remove_dir");
    }
}
