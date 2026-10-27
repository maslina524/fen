use alloc::{boxed::Box, sync::Arc, vec::Vec};
use core::{
    fmt::{self, Write},
    ops::{Index, Range, RangeFrom, RangeFull, RangeTo},
};

use crate::raw::os::{Chars, OsStr};

#[derive(Clone, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct OsString {
    inner: Vec<u16>,
}

impl OsString {
    #[must_use]
    pub const fn new() -> Self {
        Self { inner: Vec::new() }
    }

    #[must_use]
    pub const fn from_vec(v: Vec<u16>) -> Self {
        Self { inner: v }
    }

    #[must_use]
    pub const fn as_os_str(&self) -> &OsStr {
        OsStr::from_slice(self.as_slice())
    }

    #[must_use]
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            inner: Vec::with_capacity(cap),
        }
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

    pub fn pop(&mut self) -> Option<char> {
        let slice = self.as_slice();
        let last = *slice.last()?;

        let (new_len, ch) = if (0xDC00..=0xDFFF).contains(&last) {
            let high_idx = slice.len().checked_sub(2)?;
            let high = *slice.get(high_idx)? as u32;
            if !(0xD800..=0xDBFF).contains(&high) {
                return None;
            }
            let cp = 0x1_0000 + ((high - 0xD800) << 10) + (last as u32 - 0xDC00);
            (high_idx, char::from_u32(cp)?)
        } else if (0xD800..=0xDBFF).contains(&last) {
            return None;
        } else {
            (slice.len() - 1, char::from_u32(last as u32)?)
        };

        self.truncate(new_len);
        Some(ch)
    }

    pub fn ends_with<S: AsRef<OsStr>>(&self, suffix: S) -> bool {
        self.as_slice().ends_with(suffix.as_ref().as_slice())
    }

    #[must_use]
    pub fn ends_with_char(&self, suffix: char) -> bool {
        let mut buf = [0u16; 2];
        let encoded = suffix.encode_utf16(&mut buf);
        self.as_os_str().as_slice().ends_with(encoded)
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.inner.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
    }

    #[must_use]
    pub const fn as_slice(&self) -> &[u16] {
        self.inner.as_slice()
    }

    #[must_use]
    pub fn chars(&self) -> Chars<'_> {
        Chars::new(self.as_slice())
    }

    #[must_use]
    pub fn into_boxed_str(self) -> Box<OsStr> {
        let boxed: Box<[u16]> = self.as_slice().into();
        let raw: *mut [u16] = Box::into_raw(boxed);
        // SAFETY: Layout `OsStr` == [u16] (repr(transparent))
        unsafe { Box::from_raw(raw as *mut OsStr) }
    }
}

impl From<&str> for OsString {
    fn from(value: &str) -> Self {
        Self {
            inner: value.encode_utf16().collect(),
        }
    }
}

impl From<&OsStr> for OsString {
    fn from(value: &OsStr) -> Self {
        Self {
            inner: value.as_slice().to_vec(),
        }
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

impl fmt::Debug for OsString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
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
    fn from(s: &OsString) -> Self {
        let arc: Arc<[u16]> = Arc::from(s.as_slice());
        // SAFETY: Same size & alignment
        unsafe { Self::from_raw(Arc::into_raw(arc) as *const OsStr) }
    }
}

impl From<OsString> for Arc<OsStr> {
    fn from(s: OsString) -> Self {
        let arc: Arc<[u16]> = Arc::from(s.as_slice());
        // SAFETY: Same size & alignment
        unsafe { Self::from_raw(Arc::into_raw(arc) as *const OsStr) }
    }
}

impl Index<Range<usize>> for OsString {
    type Output = OsStr;

    fn index(&self, range: Range<usize>) -> &OsStr {
        OsStr::from_slice(&self.as_slice()[range])
    }
}

impl Index<RangeFrom<usize>> for OsString {
    type Output = OsStr;

    fn index(&self, range: RangeFrom<usize>) -> &OsStr {
        OsStr::from_slice(&self.as_slice()[range])
    }
}

impl Index<RangeTo<usize>> for OsString {
    type Output = OsStr;

    fn index(&self, range: RangeTo<usize>) -> &OsStr {
        OsStr::from_slice(&self.as_slice()[range])
    }
}

impl Index<RangeFull> for OsString {
    type Output = OsStr;

    fn index(&self, _: RangeFull) -> &OsStr {
        self.as_os_str()
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
    }

    #[test]
    fn osstring_deref_smoke_test() {
        let s = OsString::from("abc");
        assert_eq!(
            s.as_os_str(),
            OsStr::from_slice(&[b'a' as u16, b'b' as u16, b'c' as u16])
        );
        assert_eq!(s.as_slice().len(), 3);
    }
}
