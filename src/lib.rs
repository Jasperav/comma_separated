#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::DeriveInput;
use proc_macro2::Span;

#[proc_macro_derive(SomeDerive)]
pub fn some_derive(input: TokenStream) -> TokenStream {
    let derive: DeriveInput = syn::parse(input).unwrap();
    let struct_name = &derive.ident;
    let idents = vec![
        syn::Ident::new("name", Span::call_site()),
        syn::Ident::new("another_name", Span::call_site())
    ];

    proc_macro::TokenStream::from(quote! {
        impl #struct_name {
            fn useless() -> &'static str {
                concat!(#(stringify!(#idents)),*)
            }
        }
    })
}