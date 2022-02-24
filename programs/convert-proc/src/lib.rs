use proc_macro::TokenStream;
use crate::convertable_derive::convertable_derive;
mod convertable_derive;
mod utils;

#[proc_macro_derive(Convertable, attributes(convertable))]
pub fn convertable(input: TokenStream) -> TokenStream {
    convertable_derive(input)
}