// necessary for the TokenStream::from_str() implementation
use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::ItemStruct;

fn main() {
    // struct sample
    let s = "struct Point { x : u16 , y : u16 }";

    // create a new token stream from our string
    let tokens = TokenStream::from_str(s).unwrap();

    // build the AST: note the syn::parse2() method rather than the syn::parse() one
    // which is meant for "real" procedural macros
    let ast: ItemStruct = syn::parse2(tokens).unwrap();

    // save our struct type for future use
    let struct_type = ast.ident.to_string();
    assert_eq!(struct_type, "Point");

    // we have 2 fields
    assert_eq!(ast.fields.len(), 2);

    // syn::Fields is implementing the Iterator trait, so we can iterate through the fields
    let mut iter = ast.fields.iter();

    // this is x
    let x_field = iter.next().unwrap();
    assert_eq!(x_field.ident.as_ref().unwrap(), "x");

    // this is y
    let y_field = iter.next().unwrap();
    assert_eq!(y_field.ident.as_ref().unwrap(), "y");

    // now the most tricky part: use the quote!() macro to generate code, aka a new
    // TokenStream

    // first, build our function name: point_summation
    let function_name = format_ident!("{}_summation", struct_type.to_lowercase());

    // and our argument type. If we don't use the format ident macro, the function prototype
    // will be: pub fn point_summation (pt : "Point")
    let argument_type = format_ident!("{}", struct_type);

    // same for x and y
    let x = format_ident!("{}", x_field.ident.as_ref().unwrap());
    let y = format_ident!("{}", y_field.ident.as_ref().unwrap());

    // the quote!() macro is returning a new TokenStream. This TokenStream is returned to
    // the compiler in a "real" procedural macro
    let summation_fn = quote! {
        pub fn #function_name(pt: &#argument_type) -> u16 {
            pt.#x + pt.#y
        }
    };

    // output our function as Rust code
    println!("{}", summation_fn);

    // create the list of tokens
    // tokens type is: impl Iterator<Item = TokenStream>
    let tokens = ast.fields.iter().map(|i| quote!(pt.#i));

    // the trick is made by: 0 #(+ #tokens)*
    // which repeats the + sign on all tokens
    let summation_fn = quote! {
        pub fn #function_name(pt: &#argument_type) -> u16 {
            0 #(+ #tokens)*
        }
    };
    println!("{}", summation_fn);
}
