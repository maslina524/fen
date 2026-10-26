use core::mem::ManuallyDrop;

use crate::win32::fileapi::{FileHandle, Write};

pub mod GetStdHandle;
pub use GetStdHandle::*;

pub mod SetConsoleOutputCP;
pub use SetConsoleOutputCP::*;

pub type ConsoleHandle = ManuallyDrop<FileHandle<Write>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_handles_test() {
        GetStdHandle(StdHandle::Stdout).expect("Failed to get std handle");
        GetStdHandle(StdHandle::Stderr).expect("Failed to get std handle");
        GetStdHandle(StdHandle::Stdin).expect("Failed to get std handle");
    }
}
