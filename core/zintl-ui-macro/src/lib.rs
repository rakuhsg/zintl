use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_attribute]
pub fn composable(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let ident = input.ident.clone();

    let expanded = quote! {
        #input

        impl ::zintl::Composable for #ident {}
    };

    TokenStream::from(expanded)
}
