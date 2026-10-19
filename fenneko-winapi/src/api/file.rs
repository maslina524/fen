use core::{marker::PhantomData, ptr};

use crate::{
    Handle, NT_SUCCESS,
    raw::{HANDLE, IO_STATUS_BLOCK, NTSTATUS, NtWriteFile},
};

pub trait CanRead {}
pub trait CanWrite {}

pub struct Read;
pub struct Write;
pub struct All;

impl CanRead for Read {}
impl CanRead for All {}
impl CanWrite for Write {}
impl CanWrite for All {}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct FileHandle<Tag> {
    raw: HANDLE,
    _marker: PhantomData<Tag>,
}

impl<Tag> Handle for FileHandle<Tag> {
    fn from_raw(raw: HANDLE) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    fn as_raw(&self) -> HANDLE {
        self.raw
    }
}

/// # Errors
///
/// Mostly it always returns ``Ok()`` with the number of bytes written in it;
/// situations where ``Err()`` is returned are kernel failures, for example the disk is full.
pub fn WriteFile<T: CanWrite>(handle: &FileHandle<T>, buf: &[u8]) -> Result<usize, NTSTATUS> {
    let mut info = IO_STATUS_BLOCK::default();
    // SAFETY: The descriptor, buffer, and its length are always valid
    let status = unsafe {
        NtWriteFile(
            handle.as_raw(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            &raw mut info,
            buf.as_ptr().cast_mut().cast(),
            buf.len() as u32,
            ptr::null(),
            ptr::null(),
        )
    };

    if NT_SUCCESS!(status) {
        let written = info.Information;
        Ok(written)
    } else {
        Err(status)
    }
}

#[cfg(test)]
mod tests {
    use crate::api::{
        console::{GetStdHandle, StdHandle},
        file::WriteFile,
    };

    #[test]
    fn write_to_stdout() {
        let handle = GetStdHandle(StdHandle::Stdout).expect("Failed to get std handle");
        let string = "Hello World!\n";
        let written =
            WriteFile(&handle, string.as_bytes()).expect("Failed to write to the console");
        assert_eq!(written, string.len());
    }
}
