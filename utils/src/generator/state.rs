use std::str::FromStr;

use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    generator::{generate_field_type, generate_fields, Module},
    idl::{
        EnumVariantJsonDefinition, IdlJsonDefinition, TypeDefJsonDefinition,
        TypedefTypeJsonDefinition,
    },
    meta::Meta,
    TypesAndAccountsConfig,
};

use super::generate_doc_comments;

pub fn generate_typedef_attrs(
    name: &String,
    types_and_accounts_config: &TypesAndAccountsConfig,
    is_account: bool,
    can_copy: bool,
) -> TokenStream {
    let is_zero_copy = types_and_accounts_config.zero_copy.contains(name);
    let is_zero_copy_unsafe = types_and_accounts_config.zero_copy_unsafe.contains(name);
    let derive_attr = match (is_zero_copy, is_zero_copy_unsafe, is_account) {
        (false, false, true) => {
            if can_copy {
                quote! {
                    #[account]
                    #[derive(Copy)]
                }
            } else {
                quote! {
                    #[account]
                }
            }
        }
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
            let copy_attr = if can_copy {
                quote! { #[derive(Copy)] }
            } else {
                quote! {}
            };

            quote! {
                #[derive(AnchorDeserialize, AnchorSerialize, Clone)]
                #copy_attr
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
    idl: &IdlJsonDefinition,
    variants: &Vec<EnumVariantJsonDefinition>,
    generate_for: Module,
) -> TokenStream {
    let mut generated = TokenStream::new();

    for variant in variants {
        let generated_variant = match variant.clone() {
            EnumVariantJsonDefinition::UnitLike { docs, name } => {
                let name = TokenStream::from_str(&name.to_pascal_case()).unwrap();
                let doc = generate_doc_comments(docs);

                quote! {
                    #doc
                    #name,
                }
            }
            EnumVariantJsonDefinition::Tuple {
                docs,
                name,
                types: fields,
            } => {
                let name = TokenStream::from_str(&name.to_pascal_case()).unwrap();
                let doc = generate_doc_comments(docs);

                let mut generated_field_types = String::new();
                fields.iter().for_each(|field_type| {
                    let generated = generate_field_type(idl, field_type, generate_for);
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
            EnumVariantJsonDefinition::Struct { docs, name, fields } => {
                let name = TokenStream::from_str(&name.to_pascal_case()).unwrap();
                let doc = generate_doc_comments(docs);
                let generated_fields = generate_fields(idl, &fields, generate_for, true);

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
    idl: &IdlJsonDefinition,
    idl_state_defs: &Vec<TypeDefJsonDefinition>,
    types_and_accounts_config: &TypesAndAccountsConfig,
    meta: &Meta,
    is_account: bool,
) -> TokenStream {
    let mut generated_typedefs = TokenStream::new();
    let generate_for = if is_account {
        Module::Accounts
    } else {
        Module::Types
    };

    for typedef in idl_state_defs {
        let json_name = &typedef.name;
        let name = TokenStream::from_str(&json_name.to_pascal_case()).unwrap();

        let doc = generate_doc_comments(typedef.typedef_type.docs());

        let can_copy = meta.can_copy.get(json_name).is_some();
        let attrs = generate_typedef_attrs(
            &typedef.name.to_pascal_case(),
            types_and_accounts_config,
            is_account,
            can_copy,
        );

        let body = match &typedef.typedef_type {
            TypedefTypeJsonDefinition::Enum { variants, .. } => {
                let variants = generate_enum_variants(idl, variants, generate_for);

                quote! {
                    pub enum #name {
                        #variants
                    }
                }
            }
            TypedefTypeJsonDefinition::Struct { fields, .. } => {
                let fields = generate_fields(idl, fields, generate_for, false);
                let def = if let Some(_) = meta.can_default.get(json_name) {
                    quote! { #[derive(Default)] }
                } else {
                    quote! {}
                };

                quote! {
                    #def
                    pub struct #name {
                        #fields
                    }
                }
            }
        };

        generated_typedefs.extend(quote! {
            #doc
            #attrs
            #[derive(Debug)]
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
        idl::{
            EnumVariantJsonDefinition, FieldJsonDefinition, FieldTypeJsonDefinition,
            IdlJsonDefinition, TypeDefJsonDefinition, TypedefTypeJsonDefinition,
        },
        meta::Meta,
        TypesAndAccountsConfig,
    };

    use super::generate;

    #[test]
    fn generate_struct_type() {
        let idl = IdlJsonDefinition {
            version: "".to_owned(),
            name: "".to_owned(),
            instructions: vec![],
            accounts: vec![],
            types: vec![
                TypeDefJsonDefinition {
                    name: "OrderType".to_owned(),
                    typedef_type: TypedefTypeJsonDefinition::Struct {
                        docs: None,
                        fields: vec![],
                    },
                },
                TypeDefJsonDefinition {
                    name: "OrderParams".to_owned(),
                    typedef_type: TypedefTypeJsonDefinition::Struct {
                        docs: Some(vec![
                            String::from("This"),
                            String::from("is"),
                            String::from("doc"),
                        ]),
                        fields: vec![FieldJsonDefinition {
                            name: "orderType".to_owned(),
                            docs: Some(vec![String::from("Order type")]),
                            field_type: FieldTypeJsonDefinition::Defined {
                                defined: "OrderType".to_owned(),
                            },
                        }],
                    },
                },
            ],
            events: vec![],
            errors: vec![],
        };
        let meta = Meta::from(&idl);

        let types_and_accounts_config = TypesAndAccountsConfig::default();

        let generated = generate(&idl, &idl.types, &types_and_accounts_config, &meta, false);
        let should_be = quote! {
            use anchor_lang::prelude::*;

            #[derive(AnchorDeserialize, AnchorSerialize, Clone)]
            #[derive(Copy)]
            #[derive(Debug)]
            #[derive(Default)]
            pub struct OrderType {}

            #[doc = " This"]
            #[doc = " is"]
            #[doc = " doc"]
            #[derive(AnchorDeserialize, AnchorSerialize, Clone)]
            #[derive(Copy)]
            #[derive(Debug)]
            #[derive(Default)]
            pub struct OrderParams {
                #[doc = " Order type"]
                pub order_type: OrderType,
            }
        };

        assert_eq!(generated.to_string(), should_be.to_string());
    }

    #[test]
    fn generate_struct_type_with_default() {
        let idl = IdlJsonDefinition {
            types: vec![TypeDefJsonDefinition {
                name: "OrderParams".to_owned(),
                typedef_type: TypedefTypeJsonDefinition::Struct {
                    docs: None,
                    fields: vec![FieldJsonDefinition {
                        name: "orderId".to_owned(),
                        docs: None,
                        field_type: FieldTypeJsonDefinition::Primitive("u8".to_owned()),
                    }],
                },
            }],
            ..Default::default()
        };
        let meta = Meta::from(&idl);

        let types_and_accounts_config = TypesAndAccountsConfig::default();

        let generated = generate(&idl, &idl.types, &types_and_accounts_config, &meta, false);
        let should_be = quote! {
            use anchor_lang::prelude::*;

            #[derive(AnchorDeserialize, AnchorSerialize, Clone)]
            #[derive(Copy)]
            #[derive(Debug)]
            #[derive(Default)]
            pub struct OrderParams {
                pub order_id: u8,
            }
        };

        assert_eq!(generated.to_string(), should_be.to_string());
    }

    #[test]
    fn generate_zero_copy_enum_type() {
        let typedef = TypeDefJsonDefinition {
            name: "OrderParams".to_owned(),
            typedef_type: TypedefTypeJsonDefinition::Enum {
                docs: None,
                variants: vec![EnumVariantJsonDefinition::Tuple {
                    docs: None,
                    name: "foo".to_owned(),
                    types: vec![
                        FieldTypeJsonDefinition::Primitive("u8".to_owned()),
                        FieldTypeJsonDefinition::Option {
                            option: Box::new(FieldTypeJsonDefinition::Primitive(
                                "String".to_owned(),
                            )),
                        },
                    ],
                }],
            },
        };
        let idl = IdlJsonDefinition::default();
        let meta = Meta::from(&idl);

        let mut types_and_accounts_config = TypesAndAccountsConfig::default();
        types_and_accounts_config
            .zero_copy
            .push("OrderParams".to_owned());

        let generated = generate(
            &idl,
            &vec![typedef],
            &types_and_accounts_config,
            &meta,
            false,
        );
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
