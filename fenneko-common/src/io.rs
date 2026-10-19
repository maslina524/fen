use core::fmt;

use crate::raw::io;

pub struct Stdout(io::RawHandle);

impl Stdout {
    #[must_use]
    pub fn new() -> Self {
        Self(io::stdout())
    }

    #[must_use]
    pub fn write_all(&self, bytes: &[u8]) -> Option<usize> {
        io::write(&self.0, bytes)
    }
}

impl Default for Stdout {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_all(s.as_bytes()).ok_or(fmt::Error).map(|_| ())
    }
}

pub struct Stderr(io::RawHandle);

impl Stderr {
    #[must_use]
    pub fn new() -> Self {
        Self(io::stderr())
    }

    #[must_use]
    pub fn write_all(&self, bytes: &[u8]) -> Option<usize> {
        io::write(&self.0, bytes)
    }
}

impl Default for Stderr {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Write for Stderr {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_all(s.as_bytes()).ok_or(fmt::Error).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stdout_write_test() {
        let string = "Hello World\n";

        let std = Stdout::new();
        let ret = std
            .write_all(string.as_bytes())
            .expect("Failed to write to the stdout");

        assert_eq!(ret, string.len());
    }
}
