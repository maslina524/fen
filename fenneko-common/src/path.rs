use alloc::boxed::Box;
use core::fmt;

use crate::{
    os::SmolOsStr,
    raw::os::{OsStr, OsString},
};

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

#[repr(transparent)]
#[derive(Clone)]
pub struct Path {
    inner: SmolOsStr,
}

impl Path {
    #[must_use]
    pub const fn from_static(s: &'static OsStr) -> Self {
        Self {
            inner: SmolOsStr::from_static(s),
        }
    }

    #[must_use]
    pub fn as_os_str(&self) -> &OsStr {
        self.inner.as_os_str()
    }

    #[must_use]
    pub const fn as_smol_os_str(&self) -> &SmolOsStr {
        &self.inner
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.inner.as_bytes()
    }

    #[must_use]
    pub fn as_components(&self) -> Components<'_> {
        Components::new(&self.inner)
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl fmt::Debug for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

macro_rules! impl_from_for_path {
    ($($typ:ty),+ $(,)?) => {
        $(
            impl From<$typ> for Path {
                fn from(value: $typ) -> Self {
                    Self { inner: SmolOsStr::from(value) }
                }
            }
        )+
    };
}

impl_from_for_path!(&OsStr, OsString);

pub struct Components<'c> {
    inner: &'c [u8],
    pos: usize,
}

impl<'c> Components<'c> {
    #[must_use]
    pub const fn new(inner: &'c OsStr) -> Self {
        Self {
            inner: inner.as_bytes(),
            pos: 0,
        }
    }

    fn build_os_str(&self, start: usize) -> &'c OsStr {
        let slice = &self.inner[start..self.pos];
        // SAFETY: Bytes are created from OsStr, safe
        unsafe { OsStr::from_utf8_unchecked(slice) }
    }
}

impl<'c> Iterator for Components<'c> {
    type Item = &'c OsStr;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.pos;

        loop {
            let pos = self.pos;
            if pos >= self.inner.len() {
                return if pos == start {
                    None
                } else {
                    let osstr = self.build_os_str(start);
                    self.pos += 1;
                    Some(osstr)
                };
            }

            let ch = self.inner[self.pos];
            if ch == b'\\' || ch == b'/' {
                let osstr = self.build_os_str(start);
                self.pos += 1;
                return Some(osstr);
            }

            self.pos += 1;
        }
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
    fn concat_as_str_test() {
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

    #[test]
    fn path_components_test() {
        let path = Path::from_static("C:/Users/ferris/Rust/learn.md");
        let mut components = path.as_components();

        assert_eq!(components.next(), "C:".into());
        assert_eq!(components.next(), "Users".into());
        assert_eq!(components.next(), "ferris".into());
        assert_eq!(components.next(), "Rust".into());
        assert_eq!(components.next(), "learn.md".into());
        assert_eq!(components.next(), None);
    }

    #[test]
    fn path_components_non_ascii_test() {
        let path = Path::from_static("C:/Users/ferris/Изучение Раст/Урок №1.md");
        let mut components = path.as_components();

        assert_eq!(components.next(), "C:".into());
        assert_eq!(components.next(), "Users".into());
        assert_eq!(components.next(), "ferris".into());
        assert_eq!(components.next(), "Изучение Раст".into());
        assert_eq!(components.next(), "Урок №1.md".into());
        assert_eq!(components.next(), None);
    }
}
