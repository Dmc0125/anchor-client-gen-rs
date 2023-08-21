use std::str::FromStr;

use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    generator::{generate_field_type, generate_fields, Module},
    idl::{EnumVariant, Idl, TypeDef, TypedefType},
    TypesAndAccountsConfig,
};

use super::generate_doc_comments;

pub fn generate_typedef_attrs(
    name: &String,
    types_and_accounts_config: &TypesAndAccountsConfig,
    is_account: bool,
) -> TokenStream {
    let is_zero_copy = types_and_accounts_config.zero_copy.contains(name);
    let is_zero_copy_unsafe = types_and_accounts_config.zero_copy_unsafe.contains(name);
    let derive_attr = match (is_zero_copy, is_zero_copy_unsafe, is_account) {
        (false, false, true) => quote! {
            #[account]
        },
        (true, false, true) => quote! {
            #[account(zero_copy)]
            #[derive(AnchorDeserialize, AnchorSerialize)]
        },
        (false, true, true) => quote! {
            #[account(zero_copy(unsafe))]
            #[derive(AnchorDeserialize, AnchorSerialize)]
        },
        (true, false, false) => quote! {
            #[zero_copy]
            #[derive(AnchorDeserialize, AnchorSerialize)]
        },
        (false, true, false) => quote! {
            #[zero_copy(unsafe)]
            #[derive(AnchorDeserialize, AnchorSerialize)]
        },
        (false, false, false) => {
            quote! {
                #[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy)]
            }
        }
        _ => quote! {},
    };

    let has_repr_c = types_and_accounts_config.repr_c.contains(name);
    let has_repr_packed = types_and_accounts_config.repr_packed.contains(name);
    let repr_attr = match (has_repr_c, has_repr_packed) {
        (true, true) => {
            quote! {
                #[repr(C, packed)]
            }
        }
        (true, false) => {
            quote! {
                #[repr(C)]
            }
        }
        (false, true) => {
            quote! {
                #[repr(packed)]
            }
        }
        _ => quote! {},
    };

    quote! {
        #derive_attr
        #repr_attr
    }
}

pub fn generate_enum_variants(
    idl: &Idl,
    variants: &Vec<EnumVariant>,
    generate_for: Module,
) -> TokenStream {
    let mut generated = TokenStream::new();

    for variant in variants {
        let generated_variant = match variant.clone() {
            EnumVariant::UnitLike { docs, name } => {
                let name = TokenStream::from_str(&name.to_pascal_case()).unwrap();
                let doc = generate_doc_comments(docs);

                quote! {
                    #doc
                    #name,
                }
            }
            EnumVariant::Tuple {
                docs,
                name,
                types: fields,
            } => {
                let name = TokenStream::from_str(&name.to_pascal_case()).unwrap();
                let doc = generate_doc_comments(docs);

                let mut generated_field_types = String::new();
                fields.iter().for_each(|field_type| {
                    let (generated, _) = generate_field_type(idl, field_type, generate_for);
                    generated_field_types.push_str(&format!("{},", generated.to_string()));
                });
                let tuple_types = TokenStream::from_str(
                    &generated_field_types[..&generated_field_types.len() - 1],
                )
                .unwrap();

                quote! {
                    #doc
                    #name(#tuple_types),
                }
            }
            EnumVariant::Struct { docs, name, fields } => {
                let name = TokenStream::from_str(&name.to_pascal_case()).unwrap();
                let doc = generate_doc_comments(docs);
                let (generated_fields, _) = generate_fields(idl, &fields, generate_for);

                quote! {
                    #doc
                    #name {
                        #generated_fields
                    },
                }
            }
        };
        generated.extend(generated_variant);
    }

    generated
}

pub fn generate(
    idl: &Idl,
    idl_state_defs: &Vec<TypeDef>,
    types_and_accounts_config: &TypesAndAccountsConfig,
    is_account: bool,
) -> TokenStream {
    let mut generated_typedefs = TokenStream::new();
    let generate_for = if is_account {
        Module::Accounts
    } else {
        Module::Types
    };

    for typedef in idl_state_defs {
        let name = TokenStream::from_str(&typedef.name.to_pascal_case()).unwrap();

        let doc = generate_doc_comments(typedef.typedef_type.docs());
        let attrs = generate_typedef_attrs(
            &typedef.name.to_pascal_case(),
            types_and_accounts_config,
            is_account,
        );
        let body = match &typedef.typedef_type {
            TypedefType::Enum { variants, .. } => {
                let variants = generate_enum_variants(idl, variants, generate_for);
                quote! {
                    #[derive(Debug)]
                    pub enum #name {
                        #variants
                    }
                }
            }
            TypedefType::Struct { fields, .. } => {
                let (fields, has_defined) = generate_fields(idl, fields, generate_for);
                let def = if has_defined {
                    quote! {}
                } else {
                    quote! {
                        #[derive(Default)]
                    }
                };

                quote! {
                    #def
                    #[derive(Debug)]
                    pub struct #name {
                        #fields
                    }
                }
            }
        };

        generated_typedefs.extend(quote! {
            #doc
            #attrs
            #body
        });
    }

    quote! {
        use anchor_lang::prelude::*;
        #generated_typedefs
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::{
        idl::{EnumVariant, Field, FieldType, Idl, TypeDef, TypedefType},
        TypesAndAccountsConfig,
    };

    use super::generate;

    #[test]
    fn generate_struct_type() {
        let typedef = TypeDef {
            name: "OrderParams".to_owned(),
            typedef_type: TypedefType::Struct {
                docs: Some(vec![
                    String::from("This"),
                    String::from("is"),
                    String::from("doc"),
                ]),
                fields: vec![Field {
                    name: "orderType".to_owned(),
                    docs: Some(vec![String::from("Order type")]),
                    field_type: FieldType::Defined {
                        defined: "OrderType".to_owned(),
                    },
                }],
            },
        };
        let idl = Idl {
            version: "".to_owned(),
            name: "".to_owned(),
            instructions: vec![],
            accounts: vec![TypeDef {
                name: "OrderType".to_owned(),
                typedef_type: TypedefType::Enum {
                    docs: None,
                    variants: vec![],
                },
            }],
            types: vec![],
            events: vec![],
            errors: vec![],
        };

        let types_and_accounts_config = TypesAndAccountsConfig::default();

        let generated = generate(&idl, &vec![typedef], &types_and_accounts_config, false);
        let should_be = quote! {
            use anchor_lang::prelude::*;

            #[doc = " This"]
            #[doc = " is"]
            #[doc = " doc"]
            #[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy)]
            #[derive(Debug)]
            pub struct OrderParams {
                #[doc = " Order type"]
                pub order_type: crate::accounts::OrderType,
            }
        };

        assert_eq!(generated.to_string(), should_be.to_string());
    }

    #[test]
    fn generate_struct_type_with_default() {
        let typedef = TypeDef {
            name: "OrderParams".to_owned(),
            typedef_type: TypedefType::Struct {
                docs: None,
                fields: vec![Field {
                    name: "orderId".to_owned(),
                    docs: None,
                    field_type: FieldType::Primitive("u8".to_owned()),
                }],
            },
        };
        let idl = Idl::default();

        let types_and_accounts_config = TypesAndAccountsConfig::default();

        let generated = generate(&idl, &vec![typedef], &types_and_accounts_config, false);
        let should_be = quote! {
            use anchor_lang::prelude::*;

            #[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy)]
            #[derive(Default)]
            #[derive(Debug)]
            pub struct OrderParams {
                pub order_id: u8,
            }
        };

        assert_eq!(generated.to_string(), should_be.to_string());
    }

    #[test]
    fn generate_zero_copy_enum_type() {
        let typedef = TypeDef {
            name: "OrderParams".to_owned(),
            typedef_type: TypedefType::Enum {
                docs: None,
                variants: vec![EnumVariant::Tuple {
                    docs: None,
                    name: "foo".to_owned(),
                    types: vec![
                        FieldType::Primitive("u8".to_owned()),
                        FieldType::Option {
                            option: Box::new(FieldType::Primitive("String".to_owned())),
                        },
                    ],
                }],
            },
        };
        let idl = Idl::default();

        let mut types_and_accounts_config = TypesAndAccountsConfig::default();
        types_and_accounts_config
            .zero_copy
            .push("OrderParams".to_owned());

        let generated = generate(&idl, &vec![typedef], &types_and_accounts_config, false);
        let should_be = quote! {
            use anchor_lang::prelude::*;

            #[zero_copy]
            #[derive(AnchorDeserialize, AnchorSerialize)]
            #[derive(Debug)]
            pub enum OrderParams {
                Foo(u8, Option<String>),
            }
        };

        assert_eq!(generated.to_string(), should_be.to_string());
    }
}
