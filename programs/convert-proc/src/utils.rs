use proc_macro::Literal;
use quote::__private::ext::RepToTokensExt;
use syn::{Attribute, Ident, Path};

/// Wrapper macro that will only print if the debug feature is enabled
#[macro_export]
macro_rules! println_if_debug {
    ($($arg:tt)*) => {
        #[cfg(feature="debug")]
        println!($($arg)*)
        #[cfg(not(feature="debug"))]
        {}
    }
}

pub enum ArgumentAtom {
    Literal(Literal),
    Ident(Ident)
}

// The supported ways of invoking an argument
pub enum ArgumentData {
    Atom(ArgumentAtom),
    List(Vec<ArgumentAtom>),
    Map(Vec<(Literal, ArgumentAtom)>)
}

#[derive(Debug)]
pub struct Arguments {
    arguments: Vec<(String, ArgumentData)>,
    path: Path,
}

/// A function to parse the TokenStream inside each attribute into the
/// arguments that the macro logic can work with
pub fn parse_arguments(attributes: Vec<Attribute>) -> Vec<Arguments> {
    println_if_debug!("Parsing {} attributes", attributes.len());
    let arguments: Vec<Arguments> = Vec::new();
    for attribute in attributes {
        println_if_debug!("Parsing attribute {:?}", attribute);
        let found_arguments_inner: Vec<(String, ArgumentData)> = Vec::new();


        let arguments = Arguments {
            path: attribute.path,
            arguments: found_arguments_inner,
        };
        println_if_debug!("Found arguments {:?}", arguments);
    }
    arguments
}