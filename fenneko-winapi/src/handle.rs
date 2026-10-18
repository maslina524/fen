use crate::raw::HANDLE;

pub trait Handle {
    fn from_raw(val: HANDLE) -> Self;
    fn into_raw(self) -> HANDLE;
    fn as_raw(&self) -> HANDLE;
}

pub struct HeapHandle(HANDLE);

impl Handle for HeapHandle {
    fn from_raw(val: HANDLE) -> Self {
        Self(val)
    }

    fn into_raw(self) -> HANDLE {
        self.0
    }

    fn as_raw(&self) -> HANDLE {
        self.0
    }
}
