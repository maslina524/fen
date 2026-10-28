#![allow(
    clippy::upper_case_acronyms,
    non_upper_case_globals,
    non_camel_case_types,
    reason = "Needed for names matching the documentation"
)]

pub mod kernel32;
pub mod ntdll;

pub mod types;
pub use types::*;

pub mod peb;
pub use peb::PEB;

#[macro_export]
macro_rules! link {
    ($library:literal $($link_name:literal)? $(#[$doc:meta])? fn $($function:tt)*) => (
        #[link(name = $library)]
        unsafe extern "system" {
            $(#[link_name=$link_name])?
            pub fn $($function)*;
        }
    )
}
