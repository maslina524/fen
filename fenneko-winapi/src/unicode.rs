use alloc::vec::Vec;
use core::{error::Error, fmt, ptr, slice};

use crate::raw::{UNICODE_STRING, ntdll};

#[derive(Debug, Clone, Copy)]
pub struct ConvertDosPathToNtPathError;

impl fmt::Display for ConvertDosPathToNtPathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for ConvertDosPathToNtPathError {}

pub struct UnicodeString(UNICODE_STRING);

impl UnicodeString {
    /// # Errors
    ///
    /// Returns `Err` when it fails to convert DOS path to NT path.
    pub fn from_path(s: &[u16]) -> Result<Self, ConvertDosPathToNtPathError> {
        let mut buf: Vec<u16> = Vec::with_capacity(s.len() + 1);
        buf.extend_from_slice(s);
        buf.push(0);

        let mut unicode_raw = UNICODE_STRING::default();
        let ret = Self::dos_path_to_nt_path(&buf, &mut unicode_raw);
        if ret {
            Ok(Self(unicode_raw))
        } else {
            Err(ConvertDosPathToNtPathError)
        }
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.0.Length as usize / size_of::<u16>()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub const fn as_slice(&self) -> &[u16] {
        let inner = &self.0;
        // SAFETY: Method `len` returns the number of characters, not
        // bytes, safe
        unsafe { slice::from_raw_parts(inner.Buffer, self.len()) }
    }

    #[must_use]
    pub const fn as_inner_ptr(&self) -> *const UNICODE_STRING {
        &raw const self.0
    }

    #[must_use]
    pub const fn as_inner_mut_ptr(&mut self) -> *mut UNICODE_STRING {
        &raw mut self.0
    }

    fn dos_path_to_nt_path(s: &[u16], unicode_raw: &mut UNICODE_STRING) -> bool {
        // SAFETY: `&[u16]` slice may be incorrect, mostly this shouldn't happen
        // since in common we convert valid Paths to `&[u16]`
        let ret = unsafe {
            ntdll::RtlDosPathNameToNtPathName_U(
                s.as_ptr(),
                unicode_raw,
                ptr::null_mut(),
                ptr::null_mut(),
            )
        };
        ret.into_bool()
    }
}

impl Drop for UnicodeString {
    fn drop(&mut self) {
        let inner = &mut self.0;
        // SAFETY: `UNICODE_STRING` is valid, drop is called only once, the
        // function returns `()`
        unsafe { ntdll::RtlFreeUnicodeString(inner) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unicode_new_path_test() {
        let text = "C:/Users/";
        let vec: Vec<u16> = text.encode_utf16().collect();

        let unicode = UnicodeString::from_path(&vec).expect("Failet to alloc unicode string");
        let slice = unicode.as_slice();

        let string = String::from_utf16_lossy(slice);
        assert_eq!(string, r"\??\C:\Users\");
    }
}
