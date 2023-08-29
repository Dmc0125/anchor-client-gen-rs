use std::str::FromStr;

use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::quote;
use solana_sdk::hash;

use crate::idl::{FieldJsonDefinition, IdlJsonDefinition, InstructionAccountJsonDefinition};

use super::{generate_field_type, Module};

fn generate_discriminator(name: &String) -> TokenStream {
    let preimage = format!("global:{}", name);

    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&hash::hash(preimage.as_bytes()).to_bytes()[..8]);

    let mut discriminator_str = String::new();
    bytes.iter().for_each(|byte| {
        discriminator_str.push_str(&format!("{},", byte.to_string()));
    });

    TokenStream::from_str(&format!("[{}]", discriminator_str)).unwrap()
}

fn generate_accounts(
    idl_accounts: &Vec<InstructionAccountJsonDefinition>,
) -> (TokenStream, TokenStream) {
    let mut accounts_struct_props = TokenStream::new();
    let mut accounts_metas_elements = TokenStream::new();

    for idl_account in idl_accounts {
        let name = TokenStream::from_str(&idl_account.name.to_snake_case()).unwrap();
        let is_mut = idl_account.is_mut;
        let is_signer = idl_account.is_signer;

        accounts_struct_props.extend(quote! {
            pub #name: ::anchor_lang::prelude::Pubkey,
        });
        accounts_metas_elements.extend(quote! {
            AccountMeta {
                pubkey: accounts.#name,
                is_writable: #is_mut,
                is_signer: #is_signer,
            },
        });
    }

    (accounts_struct_props, accounts_metas_elements)
}

fn generate_args(
    idl: &IdlJsonDefinition,
    idl_args: &Vec<FieldJsonDefinition>,
) -> (TokenStream, TokenStream) {
    let mut fn_data_args = TokenStream::new();
    let mut data_props = TokenStream::new();

    for arg in idl_args {
        let name = TokenStream::from_str(&arg.name.to_snake_case()).unwrap();
        let field_type = generate_field_type(idl, &arg.field_type, Module::Instructions);

        data_props.extend(quote! {
            #name,
        });
        fn_data_args.extend(quote! {
            #name: #field_type,
        });
    }

    (fn_data_args, data_props)
}

pub fn generate(idl: &IdlJsonDefinition) -> TokenStream {
    let mut generated = TokenStream::new();

    for instruction in &idl.instructions {
        let name_pascal_case = instruction.name.to_pascal_case();
        let ix_struct_name = TokenStream::from_str(&name_pascal_case).unwrap();
        let data_struct_name = TokenStream::from_str(&format!("{}Data", name_pascal_case)).unwrap();
        let accounts_struct_name =
            TokenStream::from_str(&format!("{}Accounts", instruction.name).to_pascal_case())
                .unwrap();

        let discriminator = generate_discriminator(&instruction.name.to_snake_case());
        let (accounts_struct_props, accounts_metas_elements) =
            generate_accounts(&instruction.accounts);

        let fn_accounts_arg = if instruction.accounts.len() > 0 {
            quote! {
                accounts: #accounts_struct_name,
            }
        } else {
            quote! {}
        };

        let (fn_data_args, data_props) = generate_args(idl, &instruction.args);

        let generated_ix = quote! {
            #[derive(Default, Clone)]
            pub struct #accounts_struct_name {
                #accounts_struct_props
            }

            #[derive(::anchor_lang::prelude::AnchorSerialize)]
            struct #data_struct_name {
                discriminator: [u8; 8],
                #fn_data_args
            }

            pub struct #ix_struct_name ();

            impl #ix_struct_name {
                pub const DISCRIMINATOR: [u8; 8] = #discriminator;

                pub fn new(
                    #fn_accounts_arg
                    #fn_data_args
                ) -> Instruction {
                    let data = #data_struct_name {
                        discriminator: Self::DISCRIMINATOR,
                        #data_props
                    };

                    let accounts_metas: Vec<AccountMeta> = vec![
                        #accounts_metas_elements
                    ];

                    Instruction::new_with_borsh(crate::id(), &data, accounts_metas)
                }

                pub fn new_with_remaining_accounts(
                    #fn_accounts_arg
                    #fn_data_args
                    remaining_accounts: &Vec<AccountMeta>,
                ) -> Instruction {
                    let data = #data_struct_name {
                        discriminator: Self::DISCRIMINATOR,
                        #data_props
                    };

                    let mut accounts_metas: Vec<AccountMeta> = vec![
                        #accounts_metas_elements
                    ];

                    remaining_accounts.iter().for_each(|meta| {
                        accounts_metas.push(meta.clone());
                    });

                    Instruction::new_with_borsh(crate::id(), &data, accounts_metas)
                }
            }
        };

        generated.extend(generated_ix);
    }

    quote! {
        use anchor_lang::{
            prelude::{AccountMeta, borsh},
            solana_program::instruction::Instruction,
        };

        #generated
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use crate::idl::{
        FieldJsonDefinition, FieldTypeJsonDefinition, IdlJsonDefinition,
        InstructionAccountJsonDefinition, InstructionJsonDefinition, TypeDefJsonDefinition,
        TypedefTypeJsonDefinition,
    };

    use super::{generate, generate_discriminator};

    #[test]
    fn generate_instruction_with_accounts_and_args() {
        let mut idl = IdlJsonDefinition::default();
        idl.types.push(TypeDefJsonDefinition {
            name: "OrderParams".to_owned(),
            typedef_type: TypedefTypeJsonDefinition::Struct {
                docs: None,
                fields: vec![],
            },
        });
        idl.instructions.push(InstructionJsonDefinition {
            name: "placePerpOrder".to_owned(),
            accounts: vec![InstructionAccountJsonDefinition {
                name: "state".to_owned(),
                is_mut: true,
                is_signer: true,
            }],
            args: vec![FieldJsonDefinition {
                name: "orderParams".to_owned(),
                docs: None,
                field_type: FieldTypeJsonDefinition::Defined {
                    defined: "OrderParams".to_owned(),
                },
            }],
        });

        let generated = generate(&idl);

        let discriminator = generate_discriminator(&"place_perp_order".to_owned());
        let should_be = quote! {
            use anchor_lang::{
                prelude::{AccountMeta, borsh},
                solana_program::instruction::Instruction,
            };

            #[derive(Default, Clone)]
            pub struct PlacePerpOrderAccounts {
                pub state: ::anchor_lang::prelude::Pubkey,
            }

            #[derive(::anchor_lang::prelude::AnchorSerialize)]
            struct PlacePerpOrderData {
                discriminator: [u8; 8],
                order_params: crate::types::OrderParams,
            }

            pub struct PlacePerpOrder();

            impl PlacePerpOrder {
                pub const DISCRIMINATOR: [u8; 8] = #discriminator;

                pub fn new(
                    accounts: PlacePerpOrderAccounts,
                    order_params: crate::types::OrderParams,
                ) -> Instruction {
                    let data = PlacePerpOrderData {
                        discriminator: Self::DISCRIMINATOR,
                        order_params,
                    };

                    let accounts_metas: Vec<AccountMeta> = vec![
                        AccountMeta {
                            pubkey: accounts.state,
                            is_writable: true,
                            is_signer: true,
                        },
                    ];

                    Instruction::new_with_borsh(crate::id(), &data, accounts_metas)
                }

                pub fn new_with_remaining_accounts(
                    accounts: PlacePerpOrderAccounts,
                    order_params: crate::types::OrderParams,
                    remaining_accounts: &Vec<AccountMeta>,
                ) -> Instruction {
                    let data = PlacePerpOrderData {
                        discriminator: Self::DISCRIMINATOR,
                        order_params,
                    };

                    let mut accounts_metas: Vec<AccountMeta> = vec![
                        AccountMeta {
                            pubkey: accounts.state,
                            is_writable: true,
                            is_signer: true,
                        },
                    ];

                    remaining_accounts.iter().for_each(|meta| {
                        accounts_metas.push(meta.clone());
                    });

                    Instruction::new_with_borsh(crate::id(), &data, accounts_metas)
                }
            }
        };

        assert_eq!(generated.to_string(), should_be.to_string());
    }
}
