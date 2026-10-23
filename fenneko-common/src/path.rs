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
    pub fn new<T: AsRef<OsStr> + ?Sized>(inner: [&'cp T; N]) -> Self {
        Self {
            inner: inner.map(AsRef::as_ref),
        }
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
            osstring.push_os_str(item);
        }

        osstring.into_boxed_str()
    }

    #[must_use]
    pub fn as_os_str<'o>(&self, buf: &'o mut [u8]) -> Option<&'o OsStr> {
        let mut pos = 0;
        for item in self.as_slice() {
            let bytes = item.as_bytes();
            let end = pos + bytes.len();
            if end > buf.len() {
                return None;
            }
            buf[pos..end].copy_from_slice(bytes);
            pos = end;
        }
        // SAFETY: we copied valid UTF-16 byte data from valid OsStr's
        Some(unsafe { OsStr::from_utf16_unchecked(&buf[..pos]) })
    }

    fn byte_len(&self) -> usize {
        self.as_slice()
            .iter()
            .map(|s| s.as_bytes().len())
            .sum()
    }

    pub fn with_os_str<R>(&self, f: impl FnOnce(&OsStr) -> R) -> R {
        // SAFETY: buffer size is chosen using the same length metric
        // that `as_os_str` uses
        unsafe {
            match self.byte_len() {
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

    #[inline(never)]
    unsafe fn with_buf_unchecked<const SIZE: usize, R>(
        &self,
        f: impl FnOnce(&OsStr) -> R,
    ) -> R {
        assert!(self.byte_len() <= SIZE);

        let mut buf = [0u8; SIZE];
        let os_str = self
            .as_os_str(&mut buf)
            .expect("buffer sized using ConcatPath::byte_len");

        f(os_str)
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

impl From<&str> for Path {
    fn from(value: &str) -> Self {
        Self { inner: SmolOsStr::from(value) }
    }
}

impl AsRef<Self> for Path {
    fn as_ref(&self) -> &Self {
        self
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
    inner: OsString,
}

impl PathBuf {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            inner: OsString::new(),
        }
    }

    #[must_use]
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            inner: OsString::with_capacity(cap),
        }
    }

    #[must_use]
    pub fn as_path(&self) -> Path {
        Path::from(self.inner.as_os_str())
    }

    #[must_use]
    pub const fn as_os_str(&self) -> &OsStr {
        self.inner.as_os_str()
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
        self.inner.ends_with_char('\\') || self.inner.ends_with_char('/')
    }

    pub fn pop_trailing_sep(&mut self) -> bool {
        if !self.has_trailing_sep() {
            return false;
        }
        self.inner.pop().is_some()
    }

    #[must_use]
    pub const fn as_components(&self) -> Components<'_> {
        Components::new(self.inner.as_os_str())
    }

    pub fn pop(&mut self) -> Option<Path> {
        while self.pop_trailing_sep() {}

        let units = self.inner.as_slice();
        let mut pos = 1;
        loop {
            if pos > units.len() {
                return None;
            }
            let ch = units[units.len() - pos];
            if ch == b'/' as u16 || ch == b'\\' as u16 {
                let idx = units.len() - pos;
                let path = Path::from(&self.inner[idx + 1..]);
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
        for i in item.as_components() {
            if i == ".." {
                self.pop();
                continue;
            }
            if i != "." {
                if !self.has_trailing_sep() {
                    self.inner.push('/');
                }
                self.inner.push_os_str(i);
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
                    Self { inner: $crate::raw::os::ToOsString::to_os_string(&value) }
                }
            }
        )+
    };
}

impl_from_for_pathbuf!(&OsStr, OsString);

pub struct Components<'c> {
    inner: &'c [u16],
    pos: usize,
}

impl<'c> Components<'c> {
    #[must_use]
    pub const fn new(inner: &'c OsStr) -> Self {
        Self { inner: inner.as_slice(), pos: 0 }
    }
}

impl<'c> Iterator for Components<'c> {
    type Item = &'c OsStr;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.inner.len() {
            return None;
        }
        let start = self.pos;
        while self.pos < self.inner.len() {
            let ch = self.inner[self.pos];
            if ch == b'/' as u16 || ch == b'\\' as u16 {
                let out = OsStr::from_slice(&self.inner[start..self.pos]);
                self.pos += 1;
                return Some(out);
            }
            self.pos += 1;
        }
        Some(OsStr::from_slice(&self.inner[start..self.pos]))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rem = self.inner.len() - self.pos;
        (usize::from(rem > 0), Some(rem))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn with_concat<R>(f: impl FnOnce(ConcatPath<'_, 3>) -> R) -> R {
        let p = OsString::from("/path");
        let t = OsString::from("/to");
        let h = OsString::from("/heart");
        f(ConcatPath::new([p.as_os_str(), t.as_os_str(), h.as_os_str()]))
    }

    #[test]
    fn concat_path_len_test() {
        with_concat(|concat| {
            assert_eq!(concat.len(), "/path/to/heart".chars().count());
            assert_eq!(concat.count(), 3);
        });
    }

    #[test]
    fn concat_as_os_str_test() {
        with_concat(|concat| {
            let mut buf = [0u8; 128];
            let as_str = concat.as_os_str(&mut buf).expect("Buf too small");
            assert_eq!(as_str, "/path/to/heart");
        });
    }

    #[test]
    fn concat_as_boxed_os_str_test() {
        with_concat(|concat| {
            let as_str = concat.as_boxed_os_str();
            assert_eq!(as_str.as_ref(), "/path/to/heart");
        });
    }

    #[test]
    fn concat_with_os_str() {
        with_concat(|concat| {
            concat.with_os_str(|s| assert_eq!(s, "/path/to/heart"));
        });
    }

    #[test]
    fn path_components_test() {
        let path = Path::from("C:/Users/ferris/Rust/learn.md");
        let mut components = path.as_components();

        for expected in ["C:", "Users", "ferris", "Rust", "learn.md"] {
            assert_eq!(components.next(), Some(Path::from(expected).as_os_str()));
        }
        assert_eq!(components.next(), None);
    }

    #[test]
    fn path_components_non_ascii_test() {
        let path = Path::from("C:/Users/ferris/Изучение Раст/Урок №1.md");
        let mut components = path.as_components();

        for expected in ["C:", "Users", "ferris", "Изучение Раст", "Урок №1.md"] {
            assert_eq!(components.next(), Some(Path::from(expected).as_os_str()));
        }
        assert_eq!(components.next(), None);
    }

    #[test]
    fn path_as_concat_path_test() {
        let files = [
            OsString::from("lesson-1.md"),
            OsString::from("lesson-2.md"),
            OsString::from("lesson-3.md"),
            OsString::from("video-lesson.mp4"),
        ];
        let path = Path::from("C:/Users/ferris/Rust/");

        for file in &files {
            let concated = path.as_concat_path(file.as_os_str());
            let expected = format!("{}{file}", path.as_os_str());
            assert_eq!(concated.as_boxed_os_str().as_ref(), expected.as_str());
        }
    }

    #[test]
    fn pathbuf_pop_join_test() {
        let pathbuf = PathBuf::from(OsString::from("C:/Users/ferris"));
        assert_eq!(pathbuf.as_os_str(), Path::from("C:/Users/ferris").as_os_str());

        let mut pathbuf = pathbuf.join(&Path::from("Rust"));
        assert_eq!(pathbuf.as_os_str(), Path::from("C:/Users/ferris/Rust").as_os_str());

        let poped = pathbuf.pop();
        assert_eq!(poped, Some(Path::from("Rust")));
        assert_eq!(pathbuf.as_os_str(), Path::from("C:/Users/ferris").as_os_str());
    }

    #[test]
    fn pathbuf_super_join_test() {
        let pathbuf = PathBuf::from(OsString::from("C:/Users/ferris"));
        assert_eq!(pathbuf.as_os_str(), Path::from("C:/Users/ferris").as_os_str());

        let pathbuf = pathbuf.join(&Path::from("Rust"));
        assert_eq!(pathbuf.as_os_str(), Path::from("C:/Users/ferris/Rust").as_os_str());

        let pathbuf = pathbuf.join(&Path::from("../C"));
        assert_eq!(pathbuf.as_os_str(), Path::from("C:/Users/ferris/C").as_os_str());

        let pathbuf = pathbuf.join(&Path::from("../../Edmund"));
        assert_eq!(pathbuf.as_os_str(), Path::from("C:/Users/Edmund").as_os_str());

        let pathbuf = pathbuf.join(&Path::from("./Mewgenics"));
        assert_eq!(pathbuf.as_os_str(), Path::from("C:/Users/Edmund/Mewgenics").as_os_str());
    }
}