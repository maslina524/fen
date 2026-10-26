import!(GetStdHandle, SetConsoleOutputCP);

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
