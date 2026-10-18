use core::marker::PhantomData;

use crate::{Handle, raw::HANDLE};

pub struct Read;
pub struct Write;
pub struct All;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct FileHandle<Tag> {
    raw: HANDLE,
    _marker: PhantomData<Tag>
}

impl<Tag> Handle for FileHandle<Tag> {
    fn from_raw(raw: HANDLE) -> Self {
        Self { raw, _marker: PhantomData }
    }

    fn as_raw(&self) -> HANDLE {
        self.raw
    }
}