mod database;

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

    let store_config = get_store_config(attributes)?;

    let mut primary_key_column = store_config.primary_key;

    let struct_identifier = &input.ident;

    let table_name = match store_config.table {
        None => {struct_identifier.to_string().to_lowercase()}
        Some(name) => {name}
    };

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

    let mut column_names: Vec<String> = Vec::new();
    let mut field_idents: Vec<syn::Ident> = Vec::new();

    let field_configs = get_field_configs(&fields)?;

    for config in field_configs {
        if config.skip {
            continue
        }

        let column_name = match &config.column_name {
            Some(name) => name.clone(),
            None => config.ident.to_string()
        };

        if config.primary_key {
            if primary_key_column.is_some() {
                return Err(syn::Error::new_spanned(
                    struct_identifier,
                    "Cannot set primary key on both config and field",
                ))
            } else { primary_key_column = Some(column_name.clone())}
        }

        column_names.push(column_name);

        field_idents.push(config.ident);
    }

    let primary_key_column = primary_key_column.ok_or_else(|| {
        syn::Error::new_spanned(
            struct_identifier,
            "No primary key in this struct",
        )
    })?;

    if !column_names.contains(&primary_key_column) {
        return Err(syn::Error::new_spanned(
            struct_identifier,
            format!("primary key `{}` is not one of the stored columns", primary_key_column),
        ));
    }

    let insert_columns = column_names.join(", ");

    let placeholders = (1..=column_names.len())
        .map(|i| format!("?{}", i))
        .collect::<Vec<_>>()
        .join(", ");

    let update_assignments = column_names
        .iter()
        .filter(|column| *column != &primary_key_column)
        .map(|column| format!("{column} = excluded.{column}"))
        .collect::<Vec<_>>()
        .join(", ");

    if update_assignments.is_empty() {
        return Err(syn::Error::new_spanned(
            struct_identifier,
            "Store requires at least one non-primary-key column for update assignments",
        ));
    }

    let save_query = format!("INSERT INTO {table_name} ({insert_columns}) VALUES ({placeholders}) \
    ON CONFLICT ({primary_key_column}) DO UPDATE SET ({update_assignments})");

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