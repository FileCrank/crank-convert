extern crate proc_macro;
use proc_macro::{Literal, TokenStream};
use quote::quote;
use syn::{DeriveInput, FieldsNamed, FieldsUnnamed, Ident, parse_macro_input};

struct ConvertableOpts {
    pub ident: Ident,
    pub name: Literal,
    pub extensions: Vec<Literal>,
    pub conversions: Vec<(Ident, Ident)>
}

/*
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

 */

pub fn convertable_derive(input: TokenStream) -> TokenStream {
    let DeriveInput { data, ident, attrs, .. } = parse_macro_input!(input);
    let args = parse_macro_input!(attrs as AttributeArgs);

    for attr in attrs {
        println!("Attr.path is {:?}", attr.path);
        println!("Attr tokens are {:?}", attr.tokens);
    }

    //println!("Attributes are {:?}", derive_input.attrs);
    /*
    match derive_input.data {

    }

    println!("{}", description);
    */
    let output = quote! {
      impl Convertable for #ident {

        }
    };

    output.into()
}