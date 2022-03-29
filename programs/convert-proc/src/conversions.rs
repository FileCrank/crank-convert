use proc_macro::{Ident, TokenStream, TokenTree};
use std::collections::HashMap;
use syn::group::parse_braces;


pub struct Conversion {
    pub to: Ident,
    pub from: HashMap<Ident, Ident>
}


pub fn generate_conversions(input: TokenStream) -> TokenStream {
    TokenStream::new()
}