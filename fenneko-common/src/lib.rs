#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

extern crate alloc;

pub mod prelude;
pub(crate) mod raw;

// Public api
pub mod env;
pub mod io;
pub mod macros;
pub mod os;
pub mod path;
