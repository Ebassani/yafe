use proc_macro::{TokenStream};
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Field, Fields, LitStr};
use syn::punctuated::Punctuated;
use syn::token::Comma;

#[derive(Default, Debug)]
struct StoreConfig {
    table: Option<String>,
    primary_key: Option<String>
}

#[derive(Debug)]
struct FieldConfig {
    ident: syn::Ident,
    column_name: Option<String>,
    primary_key: bool,
    skip: bool
}

#[proc_macro_derive(Store, attributes(store))]
pub fn derive_store(token_stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(token_stream as DeriveInput);

    match expand_derive_store(&input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.into_compile_error().into()
    }
}

fn expand_derive_store(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let attributes = &input.attrs;

    let store_config = get_store_config(attributes);

    let struct_identifier = &input.ident;

    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(named_fields) => &named_fields.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    struct_identifier,
                    "Store can only be derived for structs with named fields",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                struct_identifier,
                "Store can only be derived for structs",
            ))
        }
    };

    let field_names: Vec<String> = fields.iter().map(|field| field.ident.as_ref().unwrap().to_string()).collect();

    let field_configs = get_field_configs(&fields)?;

    Ok(quote! {
        impl #struct_identifier {
            pub fn list() {
                let field_names = [#(#field_names),*];

                field_names.iter().for_each(|name| {
                    println!("{}", name);
                });
            }
        }
    })
}

fn get_store_config(attrs: &[Attribute]) -> syn::Result<StoreConfig> {
    let mut config = StoreConfig::default();

    for attr in attrs {
        if !attr.path().is_ident("store") { continue; }

        attr.parse_nested_meta(|metadata| {
            if metadata.path.is_ident("table") {
                let value = metadata.value()?;
                let lit: LitStr = value.parse()?;
                config.table = Some(lit.value());
                Ok(())
            } else if metadata.path.is_ident("primary_key") {
                let value = metadata.value()?;
                let lit: LitStr = value.parse()?;
                config.primary_key = Some(lit.value());
                Ok(())
            } else { Err(metadata.error("Unsupported attribute type")) }
        })?;
    }

    Ok(config)
}

fn get_field_configs(fields: &Punctuated<Field, Comma>) -> syn::Result<Vec<FieldConfig>> {
    let mut field_configs = Vec::new();

    for field in fields {
        let Some(ident) = field.ident.clone() else { continue };

        let config = parse_field_config(&field.attrs, ident)?;

        field_configs.push(config);
    }

    Ok(field_configs)
}

fn parse_field_config(attributes: &[Attribute], ident: syn::Ident) -> syn::Result<FieldConfig> {
    let mut column_name: Option<String> = None;
    let mut primary_key = false;
    let mut skip = false;

    for attr in attributes {
        if !attr.path().is_ident("store") {
            continue
        }

        attr.parse_nested_meta(|metadata| {
            if metadata.path.is_ident("column_name") {
                let value = metadata.value()?;
                let lit: LitStr = value.parse()?;
                column_name = Some(lit.value());
                Ok(())
            } else if metadata.path.is_ident("skip") {
                skip = true;
                Ok(())
            } else if metadata.path.is_ident("primary_key") {
                primary_key = true;
                Ok(())
            } else { Err(metadata.error("unsupported store attribute on field")) }
        })?;
    }

    Ok(FieldConfig {
        ident,
        column_name,
        primary_key,
        skip
    })
}