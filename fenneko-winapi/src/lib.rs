#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![allow(non_snake_case, reason = "WinAPI functions is camelCase")]

extern crate alloc;

pub mod allocator;
#[macro_use]
pub mod macros;
pub mod raw;
pub mod unicode;
pub mod win32;

use raw::HANDLE;

pub trait Handle {
    fn from_raw(val: HANDLE) -> Self;
    fn as_raw(&self) -> HANDLE;
}
