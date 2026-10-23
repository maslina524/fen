use alloc::boxed::Box;

use crate::raw::os::{OsStr, OsString};

#[repr(transparent)]
#[derive(Clone)]
pub struct ConcatPath<'cp, const N: usize> {
    inner: [&'cp OsStr; N],
}

impl<'cp, const N: usize> ConcatPath<'cp, N> {
    #[must_use]
    pub const fn new(inner: [&'cp OsStr; N]) -> Self {
        Self { inner }
    }

    #[must_use]
    pub fn as_slice(&self) -> &[&'cp OsStr] {
        self.inner.as_ref()
    }

    #[must_use]
    #[allow(clippy::unused_self)]
    pub const fn count(&self) -> usize {
        N
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.as_slice().iter().fold(0, |acc, s| acc + s.len())
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.count() == 0 || self.len() == 0
    }

    #[must_use]
    pub fn as_boxed_str(&self) -> Box<OsStr> {
        let len = self.len();
        let mut osstring = OsString::with_capacity(len);

        for item in self.as_slice() {
            osstring.push_str(item);
        }

        osstring.into_boxed_str()
    }

    #[must_use]
    pub fn as_str<'o>(&self, buf: &'o mut [u8]) -> Option<&'o OsStr> {
        let len = self.len();
        if buf.len() < len {
            return None;
        }

        let mut pos = 0;
        for item in self.as_slice() {
            let bytes = item.as_bytes();
            buf[pos..pos + bytes.len()].copy_from_slice(bytes);
            pos += bytes.len();
        }

        // SAFETY: Store valid OsStr, safe
        let osstr = unsafe { OsStr::from_utf8_unchecked(&buf[..len]) };
        Some(osstr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concat_path_len_test() {
        let concat = ConcatPath::new(["/path", "/to", "/heart"]);
        assert_eq!(concat.len(), "/path/to/heart".len());
        assert_eq!(concat.count(), 3);
    }

    #[test]
    fn as_str_test() {
        let concat = ConcatPath::new(["/path", "/to", "/heart"]);
        let mut buf = [0u8; 128];
        let as_str = concat.as_str(&mut buf).expect("Buf too small");
        assert_eq!(as_str, "/path/to/heart");
    }

    #[test]
    fn as_boxed_str_test() {
        let concat = ConcatPath::new(["/path", "/to", "/heart"]);
        let as_str = concat.as_boxed_str();
        assert_eq!(as_str.as_ref(), "/path/to/heart");
    }
}
