use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{LitStr, parse_macro_input};

const BOLD: &str = "\x1b[1m";
const BOLD_OUT: &str = "\x1b[22m";

pub fn imp(input: TokenStream) -> TokenStream {
    let token = parse_macro_input!(input as LitStr);
    let value = token
        .value()
        .replace("[bold]", BOLD)
        .replace("[/bold]", BOLD_OUT);

    let new_lit = LitStr::new(&value, token.span());
    new_lit.into_token_stream().into()
}
