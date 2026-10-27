use core::ptr;

use crate::{
    Handle as _,
    ntstatus::Ntstatus,
    raw::{IO_STATUS_BLOCK, ntdll},
    win32::fileapi::{CanWrite, FileHandle},
};

/// # Errors
///
/// Mostly it always returns ``Ok()`` with the number of bytes written
/// in it; situations where ``Err()`` is returned are kernel failures,
/// for example the disk is full.
pub fn WriteFile<T: CanWrite>(handle: &FileHandle<T>, buf: &[u8]) -> Result<usize, Ntstatus> {
    let mut info = IO_STATUS_BLOCK::default();
    let mut offset = 0;

    // SAFETY: The descriptor, buffer, and its length are always valid
    let status = unsafe {
        ntdll::NtWriteFile(
            handle.as_raw(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            &raw mut info,
            buf.as_ptr().cast_mut().cast(),
            buf.len() as u32,
            &raw mut offset,
            ptr::null(),
        )
    };

    if status.is_success() {
        let written = info.Information;
        Ok(written)
    } else {
        Err(status)
    }
}
