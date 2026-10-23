use alloc::boxed::Box;
use core::fmt;

use crate::{
    os::SmolOsStr,
    raw::{
        env::consts::MAX_PATH_LEN,
        os::{OsStr, OsString},
    },
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
    pub fn as_boxed_os_str(&self) -> Box<OsStr> {
        let len = self.len();
        let mut osstring = OsString::with_capacity(len);

        for item in self.as_slice() {
            osstring.push_str(item);
        }

        osstring.into_boxed_str()
    }

    #[must_use]
    pub fn as_os_str<'o>(&self, buf: &'o mut [u8]) -> Option<&'o OsStr> {
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

    pub fn with_os_str<R>(&self, f: impl FnOnce(&OsStr) -> R) -> R {
        // SAFETY: Match against the correct buffer sizes
        unsafe {
            match self.len() {
                0..=24 => self.with_buf_unchecked::<24, _>(f),
                25..=32 => self.with_buf_unchecked::<32, _>(f),
                33..=64 => self.with_buf_unchecked::<64, _>(f),
                65..=128 => self.with_buf_unchecked::<128, _>(f),
                129..=256 => self.with_buf_unchecked::<256, _>(f),
                257..=512 => self.with_buf_unchecked::<512, _>(f),
                513..=1024 => self.with_buf_unchecked::<1024, _>(f),
                1025..=2048 => self.with_buf_unchecked::<2048, _>(f),
                2049..=4096 => self.with_buf_unchecked::<4096, _>(f),
                4097..=8192 => self.with_buf_unchecked::<8192, _>(f),
                8193..=16384 => self.with_buf_unchecked::<16384, _>(f),
                _ => self.with_buf_unchecked::<MAX_PATH_LEN, _>(f),
            }
        }
    }

    /// # Safety
    ///
    /// Need to guarantee that `self.len() <= SIZE`.
    #[inline(never)]
    unsafe fn with_buf_unchecked<const SIZE: usize, R>(&self, f: impl FnOnce(&OsStr) -> R) -> R {
        assert!(self.len() <= SIZE);
        let mut buf = [0u8; SIZE];
        f(self.as_os_str(&mut buf).unwrap())
    }
}

#[repr(transparent)]
#[derive(Clone, Default, PartialEq, Eq)]
pub struct Path {
    inner: SmolOsStr,
}

impl Path {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            inner: SmolOsStr::empty(),
        }
    }

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

    #[must_use]
    pub fn as_concat_path<'cp>(&'cp self, item: &'cp OsStr) -> ConcatPath<'cp, 2> {
        ConcatPath::new([self.as_os_str(), item])
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

#[derive(Clone, Default, PartialEq, Eq)]
pub struct PathBuf {
    inner: OsString
}

impl PathBuf {
    #[must_use]
    pub const fn new() -> Self {
        Self { inner: OsString::new() }
    }

    #[must_use]
    pub fn with_capacity(cap: usize) -> Self {
        Self { inner: OsString::with_capacity(cap) }
    }

    #[must_use]
    pub fn as_path(&self) -> Path {
        Path::from(self.inner.as_str())
    }

    #[must_use]
    pub fn as_os_str(&self) -> &OsStr {
        &self.inner
    }

    #[must_use]
    pub fn into_os_string(self) -> OsString {
        self.inner
    }

    #[must_use]
    pub fn as_concat_path<'cp>(&'cp self, item: &'cp OsStr) -> ConcatPath<'cp, 2> {
        ConcatPath::new([self.as_os_str(), item])
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.inner.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn has_trailing_sep(&self) -> bool {
        self.inner.ends_with(['\\', '/'])
    }

    pub fn pop_trailing_sep(&mut self) -> bool {
        if !self.has_trailing_sep() {
            return false;
        }
        self.inner.pop().is_some()
    }

    #[must_use]
    pub fn as_components(&self) -> Components<'_> {
        Components::new(&self.inner)
    }

    pub fn pop(&mut self) -> Option<Path> {
        while self.pop_trailing_sep() {}

        let bytes = self.inner.as_bytes();
        let mut pos = 1;
        loop {
            if pos > bytes.len() {
                return None;
            }
            let ch = bytes[bytes.len() - pos];
            if ch == b'\\' || ch == b'/' {
                let idx = bytes.len() - pos;
                let osstr = &self.inner[idx + 1..];
                let path: Path = osstr.into();
                self.inner.truncate(idx);
                return Some(path);
            }
            pos += 1;
        }
    }

    #[must_use]
    pub fn join<T: AsRef<Path>>(self, item: &T) -> Self {
        self.join_inner(item.as_ref())
    }

    #[must_use]
    fn join_inner(mut self, item: &Path) -> Self {
        let item_comp = item.as_components();
        self.inner.reserve(item_comp.len());

        for i in item_comp {
            if i == ".." {
                self.pop();
                continue;
            }
            if i != "." {
                if !self.has_trailing_sep() {
                    self.inner.push('/');
                }
                self.inner.push_str(i);
            }
        }

        self
    }
}

macro_rules! impl_from_for_pathbuf {
    ($($typ:ty),+ $(,)?) => {
        $(
            impl From<$typ> for PathBuf {
                fn from(value: $typ) -> Self {
                    Self { inner: ::alloc::string::ToString::to_string(&value) }
                }
            }
        )+
    };
}

impl_from_for_pathbuf!(&OsStr, OsString);

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

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rem = self.inner.len() - self.pos;
        (rem, Some(rem))
    }
}

impl ExactSizeIterator for Components<'_> {
    fn len(&self) -> usize {
        self.inner.len()
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
    fn concat_as_os_str_test() {
        let concat = ConcatPath::new(["/path", "/to", "/heart"]);
        let mut buf = [0u8; 128];
        let as_str = concat.as_os_str(&mut buf).expect("Buf too small");
        assert_eq!(as_str, "/path/to/heart");
    }

    #[test]
    fn concat_as_boxed_os_str_test() {
        let concat = ConcatPath::new(["/path", "/to", "/heart"]);
        let as_str = concat.as_boxed_os_str();
        assert_eq!(as_str.as_ref(), "/path/to/heart");
    }

    #[test]
    fn concat_with_os_str() {
        let concat = ConcatPath::new(["/path", "/to", "/heart"]);
        concat.with_os_str(|s| {
            assert_eq!(s, "/path/to/heart");
        })
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

    #[test]
    fn path_as_concat_path_test() {
        static FILES: [&OsStr; 4] = [
            "lesson-1.md",
            "lesson-2.md",
            "lesson-3.md",
            "video-lesson.mp4",
        ];
        let path = Path::from_static("C:/Users/ferris/Rust/");

        for file in FILES {
            let concated = path.as_concat_path(file);
            assert_eq!(
                concated.as_boxed_os_str().as_ref(),
                format!("{}{file}", path.as_os_str())
            );
        }
    }

    #[test]
    fn pathbuf_pop_join_test() {
        let pathbuf = PathBuf::from("C:/Users/ferris");
        assert_eq!(pathbuf.as_os_str(), "C:/Users/ferris");

        let mut pathbuf = pathbuf.join("Rust");
        assert_eq!(pathbuf.as_os_str(), "C:/Users/ferris/Rust");

        let poped = pathbuf.pop();
        assert_eq!(poped, Some(Path::from("Rust")));
        assert_eq!(pathbuf.as_os_str(), "C:/Users/ferris");
    }

    #[test]
    fn pathbuf_super_join_test() {
        let pathbuf = PathBuf::from("C:/Users/ferris");
        assert_eq!(pathbuf.as_os_str(), "C:/Users/ferris");

        let pathbuf = pathbuf.join("Rust");
        assert_eq!(pathbuf.as_os_str(), "C:/Users/ferris/Rust");

        let pathbuf = pathbuf.join("../C");
        assert_eq!(pathbuf.as_os_str(), "C:/Users/ferris/C");

        let pathbuf = pathbuf.join("../../Edmund");
        assert_eq!(pathbuf.as_os_str(), "C:/Users/Edmund");

        let pathbuf = pathbuf.join("./Mewgenics");
        assert_eq!(pathbuf.as_os_str(), "C:/Users/Edmund/Mewgenics");
    }
}
