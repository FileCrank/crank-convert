mod conversions;

extern crate proc_macro;
use proc_macro::TokenStream;
use conversions::{generate_conversions};

#[proc_macro]
pub fn conversions(item: TokenStream) -> TokenStream {
    generate_conversions(item)
}