use core::panic;
use std::{env, path::PathBuf, str::FromStr};

use proc_macro2::TokenStream;
use quote::quote;
use solana_sdk::pubkey::Pubkey;

use crate::{
    generator::{events, instructions, state},
    idl::Idl,
};

pub mod generator;
pub mod idl;

#[derive(Default, PartialEq, Debug)]
pub struct TypesAndAccountsConfig {
    /// Accounts and types that should be zero_copy
    /// Names separated by `,`.
    pub zero_copy: Vec<String>,
    /// Accounts and types that should be zero_copy(unsafe).
    /// Should be used on zero_copy accounts if anchor version is <0.27.0.
    /// Accounts in zero_copy_unsafe can not be specified in zero_copy
    /// Names separated by `,`.
    pub zero_copy_unsafe: Vec<String>,
    /// Accounts and types that should have C compatible memory representation.
    /// #[repr(C)] is default with zero_copy and zero_copy_unsafe.
    /// Names separated by `,`.
    pub repr_c: Vec<String>,
    /// Accounts and types that should have memory layout without any padding.
    /// One account can have both C and packed memory representation.
    /// Names separated by `,`.
    pub repr_packed: Vec<String>,
}

impl TypesAndAccountsConfig {
    pub fn validate(&self) {
        let mut duplicates = String::new();
        for zero_copy in &self.zero_copy {
            if self.zero_copy_unsafe.contains(zero_copy) {
                duplicates.push_str(&format!("{}, ", zero_copy.clone()));
            }
        }

        if duplicates.len() > 0 {
            panic!(
                "zero_copy and zero_copy_unsafe can not contain same identifiers. Duplicates: {}",
                &duplicates[..duplicates.len() - 2]
            )
        }
    }
}

#[derive(Default, PartialEq, Debug)]
pub struct Args {
    /// Path to <idl>.json
    pub idl_path: PathBuf,
    /// Program id
    pub program_id: String,
    /// Skip generation of Error enum
    pub skip_errors: bool,
    /// Skip generation of events
    pub skip_events: bool,
    pub types_and_accounts_config: TypesAndAccountsConfig,
}

impl Args {
    fn remove_whitespace(str: &str) -> String {
        str.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
    }

    fn parse_inside_parenthesis<'a, T: Iterator<Item = &'a str>>(
        current: &str,
        args: &mut T,
        target: &mut Vec<String>,
        name: &str,
    ) {
        match current.split("(").collect::<Vec<&str>>()[..] {
            [_, val] => {
                if val.ends_with(")") {
                    let val = &val[..val.len() - 1];
                    target.push(val.to_owned());
                    return;
                } else {
                    target.push(val.to_owned());
                }
            }
            _ => panic!("Invalid {} arg", name),
        }

        while let Some(arg) = args.next() {
            if arg.ends_with(")") {
                let val = &arg[..arg.len() - 1];
                target.push(val.to_owned());
                return;
            } else {
                target.push(arg.to_owned());
            };
        }
        panic!("Invalid {} arg", name);
    }

    pub fn parse(args: String) -> Self {
        let args_sanitized = args.replace('\"', "").replace('\n', " ");
        let mut args = args_sanitized.split(",").map(|arg| arg.trim());

        let mut idl_path: Option<PathBuf> = None;
        let mut program_id: Option<String> = None;
        let mut types_and_accounts_config = TypesAndAccountsConfig::default();
        let mut skip_errors = false;
        let mut skip_events = false;

        while let Some(arg) = args.next() {
            if arg.starts_with("idl_path") {
                match Self::remove_whitespace(arg)
                    .split("=")
                    .collect::<Vec<&str>>()[..]
                {
                    [_, path] => {
                        if !path.ends_with(".json") {
                            panic!("Idl file needs to be in JSON format")
                        }
                        let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
                        idl_path = Some(
                            PathBuf::from_str(&cargo_manifest_dir)
                                .expect("Invalid idl_path arg")
                                .join(path),
                        );
                    }
                    _ => panic!("Invalid idl_path arg"),
                }
                continue;
            }
            if arg.starts_with("program_id") {
                match Self::remove_whitespace(arg)
                    .split("=")
                    .collect::<Vec<&str>>()[..]
                {
                    [_, program_id_str] => {
                        Pubkey::from_str(program_id_str)
                            .expect("program_id is not valid public key");
                        program_id = Some(program_id_str.to_owned());
                    }
                    _ => panic!("Invalid program_id arg"),
                }
                continue;
            }

            match arg {
                "skip_errors" => {
                    skip_errors = true;
                }
                "skip_events" => {
                    skip_events = true;
                }
                _ => {
                    if arg.starts_with("zero_copy_unsafe") {
                        Self::parse_inside_parenthesis(
                            arg,
                            &mut args,
                            &mut types_and_accounts_config.zero_copy_unsafe,
                            "zero_copy_unsafe",
                        );
                    } else if arg.starts_with("zero_copy") {
                        Self::parse_inside_parenthesis(
                            arg,
                            &mut args,
                            &mut types_and_accounts_config.zero_copy,
                            "zero_copy",
                        );
                    } else if arg.starts_with("repr_c") {
                        Self::parse_inside_parenthesis(
                            arg,
                            &mut args,
                            &mut types_and_accounts_config.repr_c,
                            "repr_c",
                        );
                    } else if arg.starts_with("repr_packed") {
                        Self::parse_inside_parenthesis(
                            arg,
                            &mut args,
                            &mut types_and_accounts_config.repr_packed,
                            "repr_packed",
                        );
                    } else {
                        panic!("Invalid arg");
                    }
                }
            }
        }

        if idl_path.is_none() {
            panic!("Missing idl_path arg");
        }

        if program_id.is_none() {
            panic!("Missing program_id arg");
        }

        types_and_accounts_config.validate();

        Self {
            program_id: program_id.unwrap(),
            idl_path: idl_path.unwrap(),
            skip_errors,
            skip_events,
            types_and_accounts_config,
        }
    }
}

pub fn generate(args: Args) -> TokenStream {
    let idl = &Idl::read_idl(&args.idl_path);

    let program_id = args.program_id;

    let types = if idl.types.len() > 0 {
        let types = state::generate(idl, &idl.types, &args.types_and_accounts_config, false);
        quote! {
            pub mod types {
                #types
            }
        }
    } else {
        quote! {}
    };
    let accounts = if idl.accounts.len() > 0 {
        let accounts = state::generate(idl, &idl.accounts, &args.types_and_accounts_config, true);
        quote! {
            pub mod accounts {
                #accounts
            }
        }
    } else {
        quote! {}
    };
    let instructions = if idl.instructions.len() > 0 {
        let instructions = instructions::generate(idl);
        quote! {
            pub mod instructions {
                #instructions
            }
        }
    } else {
        quote! {}
    };

    let events = if !args.skip_events && idl.events.len() > 0 {
        let events = events::generate(idl);
        quote! {
            #events
        }
    } else {
        quote! {}
    };

    quote! {
        anchor_lang::declare_id!(#program_id);

        #types

        #accounts

        #instructions

        #events
    }
}

#[cfg(test)]
mod tests {
    use std::{env, path::PathBuf};

    use crate::{Args, TypesAndAccountsConfig};

    #[test]
    fn parse_args() {
        let args = "idl_path = \"idl.json\", program_id =\n\"NativeLoader1111111111111111111111111111111\", skip_errors,\nzero_copy_unsafe(PerpMarket, Amm, PoolBalance, InsuranceClaim),\nrepr_c(PerpMarket)".to_owned();
        let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let parsed = Args::parse(args);
        let should_be = Args {
            idl_path: PathBuf::from(cargo_manifest_dir).join("idl.json"),
            program_id: "NativeLoader1111111111111111111111111111111".to_owned(),
            skip_errors: true,
            skip_events: false,
            types_and_accounts_config: TypesAndAccountsConfig {
                zero_copy: vec![],
                zero_copy_unsafe: vec![
                    "PerpMarket".to_owned(),
                    "Amm".to_owned(),
                    "PoolBalance".to_owned(),
                    "InsuranceClaim".to_owned(),
                ],
                repr_c: vec!["PerpMarket".to_owned()],
                repr_packed: vec![],
            },
        };

        assert_eq!(parsed, should_be);
    }

    #[test]
    #[should_panic]
    fn parse_args_panic_1() {
        Args::parse("idl_path = \"idl.json\",\nzero_copy_unsafe(PerpMarket,)".to_owned());
    }

    #[test]
    #[should_panic]
    fn parse_args_panic_2() {
        Args::parse("idl_path = \"idl.json\",\nzero_copy_unsafe(PerpMarket)".to_owned());
    }

    #[test]
    #[should_panic]
    fn parse_args_panic_duplicates() {
        Args::parse(
            "idl_path = idl.json,\nzero_copy(PerpMarket),\nzero_copy_unsafe(PerpMarket), program_id =\n\"NativeLoader1111111111111111111111111111111\"".to_string(),
        );
    }
}
