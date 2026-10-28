use alloc::{boxed::Box, sync::Arc};
use core::{
    fmt::{self, Write},
    ops::{Index, Range, RangeFrom, RangeFull, RangeTo},
    ptr, slice,
};

use crate::raw::os::{OsString, ToOsString};

#[derive(PartialEq, Eq)]
#[repr(transparent)]
pub struct OsStr {
    inner: [u16],
}

impl OsStr {
    #[must_use]
    pub const fn empty() -> &'static Self {
        Self::from_slice(&[])
    }

    #[must_use]
    pub const fn from_slice(s: &[u16]) -> &Self {
        // SAFETY: Layout `OsStr` == [u16] (repr(transparent))
        unsafe { &*(ptr::from_ref::<[u16]>(s) as *const Self) }
    }

    /// # Safety
    ///
    /// You need to guarantee that the slice is valid UTF-16
    #[must_use]
    pub const unsafe fn from_utf16_unchecked(s: &[u8]) -> &Self {
        let len = s.len() / 2;
        let ptr = s.as_ptr();
        // SAFETY: Caller guarantees s is a valid UTF-16 byte buffer
        let slice = unsafe { slice::from_raw_parts(ptr.cast(), len) };
        Self::from_slice(slice)
    }

    #[must_use]
    pub const fn as_slice(&self) -> &[u16] {
        &self.inner
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        let len = size_of_val(&self.inner);
        let ptr = self.inner.as_ptr();
        // SAFETY: ptr and length are correct, safe
        unsafe { slice::from_raw_parts(ptr.cast(), len) }
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.inner.len()
    }

    #[must_use]
    pub fn is_char_boundary(&self, index: usize) -> bool {
        if index == 0 || index == self.len() {
            return true;
        }
        if index > self.len() {
            return false;
        }
        let i = self.as_slice()[index];
        !(0xDC00..=0xDFFF).contains(&i)
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn chars(&self) -> Chars<'_> {
        Chars::new(self.as_slice())
    }

    #[must_use]
    pub fn as_boxed_str(&self) -> Box<Self> {
        let boxed: Box<[u16]> = self.as_slice().into();
        let raw: *mut [u16] = Box::into_raw(boxed);
        // SAFETY: Layout `OsStr` == [u16] (repr(transparent))
        unsafe { Box::from_raw(raw as *mut Self) }
    }
}

impl fmt::Display for OsStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for ch in self.chars() {
            f.write_char(ch)?;
        }
        Ok(())
    }
}

impl fmt::Debug for OsStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('"')?;
        for ch in self.chars() {
            f.write_char(ch)?;
        }
        f.write_char('"')?;
        Ok(())
    }
}

pub struct Chars<'c> {
    iter: slice::Iter<'c, u16>,
}

impl<'c> Chars<'c> {
    pub(crate) fn new(slice: &'c [u16]) -> Self {
        Self { iter: slice.iter() }
    }

    fn next_point(&mut self) -> Option<u32> {
        let first = u32::from(*self.iter.next()?);

        if (0xD800..=0xDBFF).contains(&first) {
            let second = u32::from(*self.iter.next()?);
            Some(0x10000 + ((first - 0xD800) << 10) + (second - 0xDC00))
        } else {
            Some(first)
        }
    }
}

impl Iterator for Chars<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: OsStr stores valid UTF-16, so next_point yields valid
        // chars
        self.next_point()
            .map(|ch| unsafe { char::from_u32_unchecked(ch) })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();
        (lower.div_ceil(2), upper)
    }
}

impl ToOsString for OsStr {
    fn to_os_string(&self) -> OsString {
        OsString::from_vec(self.inner.to_vec())
    }
}

impl ToOsString for &OsStr {
    fn to_os_string(&self) -> OsString {
        OsString::from_vec(self.inner.to_vec())
    }
}

impl PartialEq<str> for OsStr {
    fn eq(&self, other: &str) -> bool {
        self.chars().eq(other.chars())
    }
}

impl AsRef<Self> for OsStr {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl From<&OsStr> for Arc<OsStr> {
    fn from(s: &OsStr) -> Self {
        let arc: Arc<[u16]> = Arc::from(s.as_slice());
        // SAFETY: Layout `OsStr` == [u16] (repr(transparent))
        unsafe { Self::from_raw(Arc::into_raw(arc) as *const OsStr) }
    }
}

impl Index<Range<usize>> for OsStr {
    type Output = Self;

    fn index(&self, range: Range<usize>) -> &Self {
        Self::from_slice(&self.as_slice()[range])
    }
}

impl Index<RangeFrom<usize>> for OsStr {
    type Output = Self;

    fn index(&self, range: RangeFrom<usize>) -> &Self {
        Self::from_slice(&self.as_slice()[range])
    }
}

impl Index<RangeTo<usize>> for OsStr {
    type Output = Self;

    fn index(&self, range: RangeTo<usize>) -> &Self {
        Self::from_slice(&self.as_slice()[range])
    }
}

impl Index<RangeFull> for OsStr {
    type Output = Self;

    fn index(&self, _: RangeFull) -> &Self {
        self
    }
}
