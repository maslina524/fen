use core::mem::ManuallyDrop;

use crate::win32::fileapi::{FileHandle, Write};

import!(GetStdHandle, SetConsoleOutputCP);

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
