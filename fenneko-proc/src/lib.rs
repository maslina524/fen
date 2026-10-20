#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use proc_macro::TokenStream;

mod ansi;

#[proc_macro]
pub fn ansi(input: TokenStream) -> TokenStream {
    ansi::imp(input)
}
