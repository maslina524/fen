#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![warn(clippy::missing_safety_doc, clippy::undocumented_unsafe_blocks)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "There's no point in them, it will make the code cleaner"
)]
#![allow(clippy::cargo_common_metadata, reason = "Not now")]

use proc_macro::TokenStream;

mod ansi;

#[proc_macro]
pub fn ansi(input: TokenStream) -> TokenStream {
    ansi::imp(input)
}