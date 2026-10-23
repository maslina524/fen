use core::{fmt::{self, Write}, ops::Deref};
use alloc::{sync::Arc, vec::Vec};

use crate::raw::os::OsStr;

#[derive(Clone, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct OsString {
    inner: Vec<u16>
}

impl OsString {
    #[must_use]
    pub const fn new() -> Self {
        Self { inner: Vec::new() }
    }

    #[must_use]
    pub fn from_vec(v: Vec<u16>) -> Self {
        Self { inner: v }
    }

    pub fn as_os_str(&self) -> &OsStr {
        OsStr::from_slice(self.as_slice())
    }

    #[must_use]
    pub fn with_capacity(cap: usize) -> Self {
        Self { inner: Vec::with_capacity(cap) }
    }

    pub fn push(&mut self, ch: char) {
        let cp = ch as u32;
        if cp <= 0xFFFF {
            self.inner.push(cp as u16);
        } else {
            let v = cp - 0x10000;
            self.inner.push(0xD800 + (v >> 10) as u16);
            self.inner.push(0xDC00 + (v & 0x3FF) as u16);
        }
    }

    pub fn push_os_str(&mut self, s: &OsStr) {
        self.inner.extend(s.as_slice());
    }

    pub const fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
    }
}

impl From<&str> for OsString {
    fn from(value: &str) -> Self {
        OsString { inner: value.encode_utf16().collect() }
    }
}

impl Deref for OsString {
    type Target = OsStr;
    fn deref(&self) -> &Self::Target {
        self.as_os_str()
    }
}

impl fmt::Display for OsString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for ch in self.chars() {
            f.write_char(ch)?;
        }
        Ok(())
    }
}

pub trait ToOsString {
    fn to_os_string(&self) -> OsString;
}

impl ToOsString for OsString {
    fn to_os_string(&self) -> OsString {
        self.clone()
    }
}

impl From<&OsString> for Arc<OsStr> {
    fn from(s: &OsString) -> Arc<OsStr> {
        let arc: Arc<[u8]> = Arc::from(s.as_bytes());
        unsafe { Arc::from_raw(Arc::into_raw(arc) as *const OsStr) }
    }
}

impl From<OsString> for Arc<OsStr> {
    fn from(s: OsString) -> Arc<OsStr> {
        let arc: Arc<[u8]> = Arc::from(s.as_bytes());
        unsafe { Arc::from_raw(Arc::into_raw(arc) as *const OsStr) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn osstring_chars_test() {
        let string = OsString::from("Привет 👋👋👋");
        let mut chars = string.chars();
        assert_eq!(chars.next(), Some('П'));
        assert_eq!(chars.next(), Some('р'));
        assert_eq!(chars.next(), Some('и'));
        assert_eq!(chars.next(), Some('в'));
        assert_eq!(chars.next(), Some('е'));
        assert_eq!(chars.next(), Some('т'));
        assert_eq!(chars.next(), Some(' '));
        assert_eq!(chars.next(), Some('👋'));
        assert_eq!(chars.next(), Some('👋'));
        assert_eq!(chars.next(), Some('👋'));
        assert_eq!(chars.next(), None);
    }
}