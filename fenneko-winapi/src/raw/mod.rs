#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#![allow(clippy::unreadable_literal)]

pub mod link;
pub use link::*;

pub mod types;
pub use types::*;

pub mod peb;
pub use peb::PEB;