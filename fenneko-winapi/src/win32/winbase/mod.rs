pub mod CreateDirectory;
pub use CreateDirectory::*;

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn create_directory_test() {
        let path: Vec<u16> = "NewDirectory".encode_utf16().collect();
        CreateDirectory(&path).expect("Failed to create dir");

        let ret = fs::exists("NewDirectory").expect("fs::exists failed");
        assert!(ret);

        fs::remove_dir("NewDirectory")
            .expect("FS:REMOVE_DIR FAILED: CreateDirectory executed successfully, but failed to remove the directory using std::fs::remove_dir");
    }
}
