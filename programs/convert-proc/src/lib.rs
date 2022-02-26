use crate::convertable_derive::convertable_derive;
use proc_macro::TokenStream;
mod convertable_derive;
mod simple_data;

#[proc_macro_derive(Convertable, attributes(convertable, converts))]
pub fn convertable(input: TokenStream) -> TokenStream {
    convertable_derive(input)
}

#[proc_macro_derive(SimpleData, attributes(inner))]
pub fn simple_data(input: TokenStream) -> TokenStream {
    TokenStream::new()
}