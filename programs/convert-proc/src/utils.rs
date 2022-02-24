use proc_macro::{Literal};
use std::any::Any;
use quote::__private::ext::RepToTokensExt;
use quote::__private::TokenTree;
use quote::ToTokens;
use syn::{Attribute, custom_punctuation_unexpected, Expr, ExprAssign, ExprPath, Ident, Path, Token};
use syn::Expr::Assign;
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input::parse;
use syn::punctuated::Pair::Punctuated;

/// Wrapper macro that will only print if the debug feature is enabled
macro_rules! println_if_debug {
    ($($arg:tt)*) => {
        #[cfg(feature="debug")]
        println!($($arg)*);

        #[cfg(not(feature="debug"))]
        {}
    }
}

#[derive(Debug)]
pub enum ArgumentAtom {
    Literal(Literal),
    Ident(Ident)
}

/// The supported ways of invoking an argument
#[derive(Debug)]
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


fn consume_argument(parse_stream: ParseStream) -> Result<Vec<(String, ArgumentData)>, syn::Error> {
    println_if_debug!("Trying to consume {}", parse_stream);
    let punctuated = parse_stream.parse_terminated::<ExprAssign, Token![,]>(ExprAssign::parse)?;
    let found_args: Vec<(String, ArgumentData)> = Vec::new();

    for it in punctuated {
        let left = it.left.to_token_stream().to_string();
        println_if_debug!("Got expr {:?}, left {}", it, left);
    }
    Ok(found_args)
}


/// A function to parse the TokenStream inside each attribute into the
/// arguments that the macro logic can work with
pub fn parse_arguments(attributes: Vec<Attribute>) -> Vec<Arguments> {
    println_if_debug!("Parsing {} attributes", attributes.len());
    let arguments: Vec<Arguments> = Vec::new();
    for attribute in attributes {
        println_if_debug!("Parsing attribute {:?}", attribute);
        let arg_result = attribute.parse_args_with(consume_argument);
        println_if_debug!("Arg result is {:?}", arg_result);
        let found_arguments_inner: Vec<(String, ArgumentData)> = Vec::new();

        /*
        let parsed_tree = parse(attribute.tokens.into());
        println!("Parsed tree is {:?}", parsed_tree);
        let arguments = Arguments {
            path: attribute.path,
            arguments: found_arguments_inner,
        };

         */
        println_if_debug!("Found arguments {:?}", arguments);
    }
    arguments
}