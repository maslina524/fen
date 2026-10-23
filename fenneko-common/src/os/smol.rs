use alloc::{borrow::Cow, string::String, sync::Arc};
use core::{ffi::CStr, mem, ops::Deref, str::Utf8Error};

use crate::raw::os::OsStr;

const INLINE_CAP: usize = 23;

#[repr(u8)]
#[derive(Clone, Copy)]
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
    _V12 = 12,
    _V13 = 13,
    _V14 = 14,
    _V15 = 15,
    _V16 = 16,
    _V17 = 17,
    _V18 = 18,
    _V19 = 19,
    _V20 = 20,
    _V21 = 21,
    _V22 = 22,
    _V23 = 23,
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

#[derive(Default, Clone)]
pub enum Repr {
    #[default]
    Empty,
    Inline {
        len: InlineLen,
        data: [u8; INLINE_CAP],
    },
    Heap(Arc<OsStr>),
    Static(&'static OsStr),
}

/// [`SmolOsStr`] only stores the string, does not modify it
#[derive(Default, Clone)]
pub struct SmolOsStr(Repr);

impl SmolOsStr {
    #[must_use]
    pub const fn empty() -> Self {
        Self(Repr::Empty)
    }

    #[must_use]
    pub const fn from_static(s: &'static str) -> Self {
        if s.is_empty() {
            Self::empty()
        } else {
            Self(Repr::Static(s))
        }
    }

    #[must_use]
    pub fn as_os_str(&self) -> &str {
        match &self.0 {
            Repr::Empty => "",
            Repr::Heap(a) => a,
            // SAFETY: `SmolStr` is created only from `str`, utf8 is always valid.
            Repr::Inline { len, data } => unsafe {
                str::from_utf8_unchecked(&data[..*len as usize])
            },
            Repr::Static(s) => s,
        }
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        match &self.0 {
            Repr::Empty => &[],
            Repr::Heap(a) => a.as_bytes(),
            Repr::Inline { len, data } => &data[..*len as usize],
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
fn str_to_arr(s: &OsStr) -> [u8; INLINE_CAP] {
    let mut out = [0u8; INLINE_CAP];
    let mut len = s.len().min(INLINE_CAP);

    while len > 0 && !s.is_char_boundary(len) {
        len -= 1;
    }

    out[..len].copy_from_slice(&s.as_bytes()[..len]);
    out
}

macro_rules! impl_from_string {
    ($($typ:ty),+ $(,)?) => {$(
        impl From<$typ> for SmolOsStr {
            fn from(value: $typ) -> Self {
                let repr = if value.len() == 0 {
                    Repr::Empty
                } else if value.len() > INLINE_CAP {
                    Repr::Heap(Arc::from(value))
                } else {
                    Repr::Inline {
                        len: (value.len() as u8).try_into().unwrap(),
                        data: str_to_arr(&value)
                    }
                };

                Self(repr)
            }
        }
    )+};
}

impl_from_string!(String, &str, Cow<'static, str>);

impl TryFrom<&CStr> for SmolOsStr {
    type Error = Utf8Error;

    fn try_from(value: &CStr) -> Result<Self, Self::Error> {
        let len = value.count_bytes();
        let repr = if len == 0 {
            Repr::Empty
        } else if len > INLINE_CAP {
            let s = value.to_str()?;
            Repr::Heap(Arc::from(s))
        } else {
            let s = value.to_str()?;
            Repr::Inline {
                len: (len as u8).try_into().unwrap(),
                data: str_to_arr(s),
            }
        };

        Ok(Self(repr))
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
