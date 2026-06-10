use proc_macro::{TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Store, attributes(store))]
pub fn derive_store(token_stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(token_stream as DeriveInput);

    let struct_identifier = &input.ident;

    let fields = match input.data {
        Data::Struct(data_struct) => match data_struct.fields {
            Fields::Named(named_fields) => named_fields.named,
            _ => {
                return syn::Error::new_spanned(
                    struct_identifier,
                    "Store can only be derived for structs with named fields",
                )
                    .to_compile_error()
                    .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(
                struct_identifier,
                "Store can only be derived for structs",
            )
                .to_compile_error()
                .into();
        }
    };

    let field_names: Vec<String> = fields.iter().map(|field| field.ident.as_ref().unwrap().to_string()).collect();

    quote! {
        impl #struct_identifier {
            pub fn list() {
                let field_names = [#(#field_names),*];

                field_names.iter().for_each(|name| {
                    println!("{}", name);
                });
            }
        }
    }.into()
}