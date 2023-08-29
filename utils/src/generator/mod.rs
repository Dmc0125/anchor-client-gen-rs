use std::str::FromStr;

use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::quote;

use crate::idl::{FieldJsonDefinition, FieldTypeJsonDefinition, IdlJsonDefinition};

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
    idl: &IdlJsonDefinition,
    field_type: &FieldTypeJsonDefinition,
    generate_for: Module,
) -> TokenStream {
    let generated = match field_type {
        FieldTypeJsonDefinition::Primitive(primitive) => {
            let primitive = TokenStream::from_str(match primitive.as_str() {
                "publicKey" => "::anchor_lang::prelude::Pubkey",
                "string" => "String",
                _ => &primitive,
            })
            .unwrap();

            quote! {
                #primitive
            }
        }
        FieldTypeJsonDefinition::Array { array } => {
            let inner_type = array.0.clone();
            let inner = generate_field_type(idl, &inner_type, generate_for);
            let count = TokenStream::from_str(&array.1.to_string()).unwrap();

            quote! {
                [#inner; #count]
            }
        }
        FieldTypeJsonDefinition::Defined { defined } => {
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

            quote! {
                #defined
            }
        }
        FieldTypeJsonDefinition::Option { option } => {
            let inner = generate_field_type(idl, option, generate_for);
            quote! {
                Option<#inner>
            }
        }
        FieldTypeJsonDefinition::Vec { vec } => {
            let inner = generate_field_type(idl, vec, generate_for);
            quote! {
                Vec<#inner>
            }
        }
    };

    generated
}

pub fn generate_fields(
    idl: &IdlJsonDefinition,
    fields: &Vec<FieldJsonDefinition>,
    generate_for: Module,
    is_enum_struct_field: bool,
) -> TokenStream {
    let mut generated = TokenStream::new();

    for field in fields {
        let name = TokenStream::from_str(&field.name.to_snake_case()).unwrap();
        let doc = generate_doc_comments(field.docs.clone());
        let field_type = generate_field_type(idl, &field.field_type, generate_for);

        let _pub = if is_enum_struct_field {
            quote! {}
        } else {
            quote! { pub }
        };

        generated.extend(quote! {
            #doc
            #_pub #name: #field_type,
        });
    }

    generated
}
