use std::str::FromStr;

use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::quote;

use crate::idl::{Field, FieldType, Idl};

pub mod events;
pub mod instructions;
pub mod state;

pub fn generate_doc_comments(docs: Option<Vec<String>>) -> TokenStream {
    match docs {
        None => quote! {},
        Some(docs) => {
            let mut generated = TokenStream::new();
            docs.iter().for_each(|comment| {
                let doc = TokenStream::from_str(&format!("/// {}", comment)).unwrap();
                generated.extend(doc);
            });
            generated
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Module {
    Accounts,
    Types,
    Instructions,
    Events,
}

impl Module {
    pub fn get_prefix(&self) -> String {
        match self {
            Self::Accounts => "crate::accounts::".to_owned(),
            Self::Types => "crate::types::".to_owned(),
            Self::Instructions => "crate::instructions::".to_owned(),
            Self::Events => "crate::events::".to_owned(),
        }
    }
}

pub fn generate_field_type(
    idl: &Idl,
    field_type: &FieldType,
    generate_for: Module,
) -> (TokenStream, bool) {
    let mut is_defined = false;
    let generated = match field_type {
        FieldType::Primitive(primitive) => {
            let primitive = if primitive == "publicKey" {
                "::anchor_lang::prelude::Pubkey"
            } else {
                primitive
            };
            let primitive = TokenStream::from_str(&primitive).unwrap();

            quote! {
                #primitive
            }
        }
        FieldType::Array { array } => {
            let inner_type = array.0.clone();
            let (inner, is_inner_defined) = generate_field_type(idl, &inner_type, generate_for);
            let count = TokenStream::from_str(&array.1.to_string()).unwrap();
            is_defined = is_inner_defined;

            quote! {
                [#inner; #count]
            }
        }
        FieldType::Defined { defined } => {
            let pascal_case = &defined.to_pascal_case();
            let defined = match idl.get_typedef_module(defined) {
                None => pascal_case.clone(),
                Some(module) => {
                    if module == generate_for {
                        pascal_case.clone()
                    } else {
                        let module_prefix = module.get_prefix();
                        let type_with_prefix = format!("{}{}", module_prefix, pascal_case);
                        type_with_prefix
                    }
                }
            };
            let defined = TokenStream::from_str(&defined).unwrap();
            is_defined = true;

            quote! {
                #defined
            }
        }
        FieldType::Option { option } => {
            let (inner, is_inner_defined) = generate_field_type(idl, option, generate_for);
            is_defined = is_inner_defined;
            quote! {
                Option<#inner>
            }
        }
        FieldType::Vec { vec } => {
            let (inner, is_inner_defined) = generate_field_type(idl, vec, generate_for);
            is_defined = is_inner_defined;
            quote! {
                Vec<#inner>
            }
        }
    };

    (generated, is_defined)
}

pub fn generate_fields(
    idl: &Idl,
    fields: &Vec<Field>,
    generate_for: Module,
) -> (TokenStream, bool) {
    let mut generated = TokenStream::new();
    let mut has_defined = false;

    for field in fields {
        let name = TokenStream::from_str(&field.name.to_snake_case()).unwrap();
        let doc = generate_doc_comments(field.docs.clone());
        let (field_type, is_defined) = generate_field_type(idl, &field.field_type, generate_for);

        if is_defined {
            has_defined = true;
        }

        generated.extend(quote! {
            #doc
            pub #name: #field_type,
        });
    }

    (generated, has_defined)
}
