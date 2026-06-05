use proc_macro::{TokenStream};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Store, attributes(store))]
pub fn derive_store(token_stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(token_stream as DeriveInput);

    let ident = &input.ident;

    match &input.data {
        _ => todo!()
    }
}