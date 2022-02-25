use crate::convertable_derive::convertable_derive;
use proc_macro::TokenStream;
mod convertable_derive;

#[proc_macro_derive(Convertable, attributes(convertable, converts))]
pub fn convertable(input: TokenStream) -> TokenStream {
    let res = convertable_derive(input);
    println!("Returning {}", res);
    syn::Resu
}

