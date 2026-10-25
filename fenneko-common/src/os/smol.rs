use alloc::sync::Arc;
use core::{mem, ops::Deref, slice};

use crate::raw::os::{OsStr, OsString};

const INLINE_CAP: usize = 11;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InlineLen {
    _V1 = 1,
    _V2 = 2,
    _V3 = 3,
    _V4 = 4,
    _V5 = 5,
    _V6 = 6,
    _V7 = 7,
    _V8 = 8,
    _V9 = 9,
    _V10 = 10,
    _V11 = 11,
}

impl TryFrom<u8> for InlineLen {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, ()> {
        if (1u8..=INLINE_CAP as u8).contains(&value) {
            // SAFETY: Enum bounds are checked, it has the same layout as u8,
            // safe
            let len = unsafe { mem::transmute::<u8, Self>(value) };
            Ok(len)
        } else {
            Err(())
        }
    }
}

#[derive(Default, Clone, PartialEq, Eq)]
pub enum Repr {
    #[default]
    Empty,
    Inline {
        len: InlineLen,
        data: [u16; INLINE_CAP],
    },
    Heap(Arc<OsStr>),
    Static(&'static OsStr),
}

/// [`SmolOsStr`] only stores the string, does not modify it
#[derive(Default, Clone, PartialEq, Eq)]
pub struct SmolOsStr(Repr);

impl SmolOsStr {
    #[must_use]
    pub const fn empty() -> Self {
        Self(Repr::Empty)
    }

    #[must_use]
    pub const fn from_static(s: &'static OsStr) -> Self {
        if s.is_empty() {
            Self::empty()
        } else {
            Self(Repr::Static(s))
        }
    }

    #[must_use]
    pub fn as_os_str(&self) -> &OsStr {
        match &self.0 {
            Repr::Empty => OsStr::empty(),
            Repr::Heap(a) => a,
            // SAFETY: `SmolStr` is created only from `str`, utf8 is always
            // valid.
            Repr::Inline { len, data } => OsStr::from_slice(&data[..*len as usize]),
            Repr::Static(s) => s,
        }
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        match &self.0 {
            Repr::Empty => &[],
            Repr::Heap(a) => a.as_bytes(),
            Repr::Inline { len, data } => {
                let utf16 = &data[..*len as usize];
                let len = mem::size_of_val(utf16);
                let ptr = utf16.as_ptr();
                // SAFETY: `SmolOsStr` is created from a valid `OsStr`, safe
                unsafe { slice::from_raw_parts(ptr.cast(), len) }
            },
            Repr::Static(s) => s.as_bytes(),
        }
    }
}

impl Deref for SmolOsStr {
    type Target = OsStr;

    fn deref(&self) -> &Self::Target {
        self.as_os_str()
    }
}

#[inline]
fn str_to_arr(s: &OsStr) -> [u16; INLINE_CAP] {
    let mut out = [0u16; INLINE_CAP];
    let mut len = s.len().min(INLINE_CAP);

    while len > 0 && !s.is_char_boundary(len) {
        len -= 1;
    }

    out[..len].copy_from_slice(&s.as_slice()[..len]);
    out
}

#[allow(clippy::fallible_impl_from)]
impl From<&OsStr> for SmolOsStr {
    fn from(value: &OsStr) -> Self {
        let repr = if value.is_empty() {
            Repr::Empty
        } else if value.len() > INLINE_CAP {
            Repr::Heap(value.into())
        } else {
            Repr::Inline {
                len: (value.len() as u8).try_into().unwrap(),
                data: str_to_arr(value),
            }
        };
        Self(repr)
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<OsString> for SmolOsStr {
    fn from(value: OsString) -> Self {
        let repr = if value.is_empty() {
            Repr::Empty
        } else if value.len() > INLINE_CAP {
            Repr::Heap(value.into())
        } else {
            Repr::Inline {
                len: (value.len() as u8).try_into().unwrap(),
                data: str_to_arr(value.as_os_str()),
            }
        };
        Self(repr)
    }
}

impl From<&str> for SmolOsStr {
    fn from(value: &str) -> Self {
        let osstring: OsString = value.into();
        Self::from(osstring)
    }
}

impl core::fmt::Display for SmolOsStr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_os_str())
    }
}

impl core::fmt::Debug for SmolOsStr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.as_os_str())
    }
}
