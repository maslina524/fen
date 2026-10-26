pub mod GetCommandLine;
pub use GetCommandLine::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_line_test() {
        let string = GetCommandLine();
        assert!(string.contains("cargo"));
    }
}
