use core::{error::Error, fmt};

use crate::raw::io;

#[derive(Debug, Clone, Copy)]
pub struct StdWriteError;

impl fmt::Display for StdWriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for StdWriteError {}

pub struct Stdout(io::RawHandle);

impl Stdout {
    #[must_use]
    pub fn new() -> Self {
        Self(io::stdout())
    }

    /// # Errors
    ///
    /// Returns `Err` when it fails to write bytes to Std handle.
    pub fn write_all(&self, bytes: &[u8]) -> Result<usize, StdWriteError> {
        io::write(&self.0, bytes).map_err(|_| StdWriteError)
    }
}

impl Default for Stdout {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_all(s.as_bytes())
            .map_err(|_| fmt::Error)
            .map(|_| ())
    }
}

pub struct Stderr(io::RawHandle);

impl Stderr {
    #[must_use]
    pub fn new() -> Self {
        Self(io::stderr())
    }

    /// # Errors
    ///
    /// Returns `Err` when it fails to write bytes to Std handle.
    pub fn write_all(&self, bytes: &[u8]) -> Result<usize, StdWriteError> {
        io::write(&self.0, bytes).map_err(|_| StdWriteError)
    }
}

impl Default for Stderr {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Write for Stderr {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_all(s.as_bytes())
            .map_err(|_| fmt::Error)
            .map(|_| ())
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
