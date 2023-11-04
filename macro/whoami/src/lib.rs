use proc_macro::TokenStream; // no need to import a specific crate for TokenStream

// Generate a compile error to output struct name
#[proc_macro_derive(WhoAmI)]
pub fn whatever_you_want(tokens: TokenStream) -> TokenStream {
    // convert the input tokens into an ast, specially from a derive
    let ast: syn::DeriveInput = syn::parse(tokens).unwrap();

    panic!("My struct name is: <{}>", ast.ident.to_string());

    // TokenStream::new()
}
