extern crate proc_macro;
use darling::{FromDeriveInput, FromField, FromMeta};
use proc_macro::{Literal, TokenStream};
use std::ops::Deref;
use quote::quote;
use syn::{DeriveInput, FieldsNamed, FieldsUnnamed, Ident, Lit, Meta, NestedMeta, parse_macro_input};
use syn::__private::Span;
use lazy_static::lazy_static;

/// Wrapper macro that will only print if the debug feature is enabled
macro_rules! println_if_debug {
    ($($arg:tt)*) => {
        #[cfg(feature="debug")]
        println!($($arg)*);

        #[cfg(not(feature="debug"))]
        {}
    }
}

#[derive(FromDeriveInput, FromField, Default)]
#[darling(default, attributes(convertable))]
struct ConvertableOpts {
    #[darling(default)]
    pub name: String,

    #[darling(multiple, default)]
    pub extension: Vec<String>,
}

#[derive(FromDeriveInput)]
#[darling(attributes(converts))]
struct ConvertsToOpts {
    #[darling(default)]
    pub to: String,

    #[darling(default)]
    pub handler: String,
}


fn gen_convertable_output(opts: ConvertableOpts) -> TokenStream {
    let output = quote!{

        // Create the HashSet for the extensions, and supported
        ::lazy_static::lazy_static! {

        };

        impl Convertable for txt {

        }
    };
    output.into()
}


pub fn convertable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);

    let convertable_opts = ConvertableOpts::from_derive_input(&input)
        .expect("Wrong options for #[convertable(...)] call");

    // TODO: #[convertable] can be through darling, but #[converts] needs to be custom
    let converts_to_opts = ConvertsToOpts::from_derive_input(&input)
        .expect("Wrong options for #[converts(...)] call");

    TokenStream::new()
}