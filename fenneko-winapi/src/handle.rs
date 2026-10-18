use crate::raw::HANDLE;

pub trait Handle {
    fn from_raw(val: HANDLE) -> Self;
    fn as_raw(&self) -> HANDLE;
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct HeapHandle(HANDLE);

impl Handle for HeapHandle {
    fn from_raw(val: HANDLE) -> Self {
        Self(val)
    }

    fn as_raw(&self) -> HANDLE {
        self.0
    }
}
