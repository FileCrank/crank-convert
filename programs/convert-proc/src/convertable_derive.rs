extern crate proc_macro;
use darling::{FromDeriveInput, FromField, FromMeta};
use lazy_static::lazy_static;
use proc_macro::{Literal, TokenStream};
use std::collections::HashMap;
use quote::{format_ident, quote};
use std::ops::Deref;
use std::str::FromStr;
use syn::__private::Span;
use syn::{
    parse_macro_input, parse_str, Attribute, DeriveInput, FieldsNamed, FieldsUnnamed, Ident, Lit,
    Meta, NestedMeta,
};

/// Wrapper macro that will only print if the debug feature is enabled
macro_rules! println_if_debug {
    ($($arg:tt)*) => {
        #[cfg(feature="debug")]
        println!($($arg)*);

        #[cfg(not(feature="debug"))]
        {}
    }
}

#[derive(FromDeriveInput)]
#[darling(attributes(convertable), forward_attrs(allow, doc, cfg))]
struct ConvertableOpts {
    pub ident: Ident,
    pub attrs: Vec<Attribute>,

    #[darling(default)]
    pub name: String,

    #[darling(multiple, default)]
    pub extension: Vec<String>,
}

struct ConvertsTo {
    pub from: Ident,
    pub handler: Ident,
}

fn gen_convertable_output(opts: ConvertableOpts) -> TokenStream {
    let ident = opts.ident;
    let name = opts.name;
    let extensions_name = format_ident!("{}_EXTENSIONS", ident);
    let extensions = opts.extension;
    // We'll use PHF to generate a static map for the extensions,
    // as we can't invoke lazy_static! in a proc macro, which would be the typical solution
    // here
    let mut phf_set = phf_codegen::Set::new();
    for extension in extensions {
        phf_set.entry(extension);
    }
    let phf_invocation = format!(
        "static {}: phf::Set<&'static str> = {}",
        extensions_name,
        phf_set.build()
    );
    let phf_parsed: proc_macro2::TokenStream = phf_invocation.parse().unwrap();
    println_if_debug!("PHF invocation is {}", phf_invocation);
    let output = quote! {
        use phf;

        // Create the Set for the extensions, and supported
        #phf_parsed;

        impl Convertable for #ident {
            const NAME: &'static str = #name;

            fn extensions(&self) -> phf::Set<&'static str> {
                #extensions_name
            }
        }

    };
    output.into()
}

fn parse_conversions(input: DeriveInput) -> HashMap<Ident, Ident> {

}

pub fn convertable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);

    let convertable_opts = ConvertableOpts::from_derive_input(&input)
        .expect("Wrong options for #[convertable(...)] call");

    /*
    // TODO: #[convertable] can be through darling, but #[converts] needs to be custom
    let converts_to_opts = ConvertsToOpts::from_derive_input(&input)
        .expect("Wrong options for #[converts(...)] call");

     */

    gen_convertable_output(convertable_opts)
}
