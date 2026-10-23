use core::{fmt::{self, Write}, mem, ops::{Index, Range, RangeFrom, RangeFull, RangeTo}, slice};
use alloc::{boxed::Box, sync::Arc};

use crate::raw::os::{OsString, ToOsString};

#[derive(PartialEq, Eq)]
#[repr(transparent)]
pub struct OsStr {
    inner: [u16]
}

impl OsStr {
    pub fn empty() -> &'static OsStr {
        Self::from_slice(&[])
    }

    pub fn from_slice(s: &[u16]) -> &OsStr {
        // SAFETY: Layout `OsStr` == [u16] (repr(transparent))
        unsafe { &*(s as *const [u16] as *const OsStr) }
    }

    pub unsafe fn from_utf16_unchecked(s: &[u8]) -> &Self {
        let len = mem::size_of_val(&s);
        let ptr = s.as_ptr();
        let slice = unsafe { slice::from_raw_parts(ptr.cast(), len) };
        Self::from_slice(slice)
    }

    pub fn as_slice(&self) -> &[u16] {
        &self.inner
    }

    pub const fn as_bytes(&self) -> &[u8] {
        let len = mem::size_of_val(&self.inner);
        let ptr = self.inner.as_ptr();
        unsafe { slice::from_raw_parts(ptr.cast(), len) }
    }

    pub const fn len(&self) -> usize {
        self.inner.len()
    }

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

    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn chars(&self) -> Chars {
        Chars::new(self.as_slice())
    }

    pub fn into_boxed_str(&self) -> Box<OsStr> {
        let boxed: Box<[u16]> = self.as_slice().into();
        let raw: *mut [u16] = Box::into_raw(boxed);
        // SAFETY: Layout `OsStr` == [u16] (repr(transparent))
        unsafe { Box::from_raw(raw as *mut OsStr) }
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
    iter: slice::Iter<'c, u16>
}

impl<'c> Chars<'c> {
    pub(crate) fn new(slice: &'c [u16]) -> Self {
        Self { iter: slice.iter() }
    }

    fn next_point(&mut self) -> Option<u32> {
        let first = *self.iter.next()? as u32;

        if (0xD800..=0xDBFF).contains(&first) {
            let second = *self.iter.next()? as u32;
            Some(0x10000 + ((first - 0xD800) << 10) + (second - 0xDC00))
        } else {
            Some(first)
        }
    }
}

impl<'c> Iterator for Chars<'c> {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: `OsStr` stores valid UTF16 and forms valid chars
        self.next_point().map(|ch| unsafe { char::from_u32_unchecked(ch) })
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
        let self_chars = self.chars();
        let other_chars = other.chars();
        if self_chars.size_hint().0 != other_chars.size_hint().0 {
            return false;
        }

        for (s, o) in self_chars.zip(other_chars) {
            if s != o {
                return false;
            }
        }

        true
    }
}

impl AsRef<OsStr> for OsStr {
    fn as_ref(&self) -> &OsStr {
        self
    }
}

impl From<&OsStr> for Arc<OsStr> {
    fn from(s: &OsStr) -> Arc<OsStr> {
        let arc: Arc<[u8]> = Arc::from(s.as_bytes());
        unsafe { Arc::from_raw(Arc::into_raw(arc) as *const OsStr) }
    }
}

impl Index<Range<usize>> for OsStr {
    type Output = OsStr;
    fn index(&self, range: Range<usize>) -> &OsStr {
        OsStr::from_slice(&self.as_slice()[range])
    }
}

impl Index<RangeFrom<usize>> for OsStr {
    type Output = OsStr;
    fn index(&self, range: RangeFrom<usize>) -> &OsStr {
        OsStr::from_slice(&self.as_slice()[range])
    }
}

impl Index<RangeTo<usize>> for OsStr {
    type Output = OsStr;
    fn index(&self, range: RangeTo<usize>) -> &OsStr {
        OsStr::from_slice(&self.as_slice()[range])
    }
}

impl Index<RangeFull> for OsStr {
    type Output = OsStr;
    fn index(&self, _: RangeFull) -> &OsStr { self }
}