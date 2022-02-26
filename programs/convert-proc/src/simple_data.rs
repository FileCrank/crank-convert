use proc_macro::{TokenStream};
use darling::{FromDeriveInput};
use quote::quote;
use syn::{parse_macro_input, Ident};

/* Helper macro to derive Deref for the simple_convertable type,
 which most file conversion types will want to use to hold their data.

 */


/*
#[derive(FromDeriveInput)]
#[darling(attributes(inner), forward_attrs(allow, doc, cfg))]
struct SimpleDataOpts {
    #[darling(default)]
    pub ident: Ident,
    #[darling(default)]
    pub inner: Ident,
}

fn simple_data_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);

    let simple_data_opts = SimpleDataOpts::from_derive_input(&input)
        .unwrap();
    let ident: Ident = simple_data_opts.ident;

    let output = quote! {
        use crate::conversions::common::simple_convertable::SimpleConvertable;

        impl<T> Deref for #ident<T> {

        }
    };

    TokenStream::new()
}

 */