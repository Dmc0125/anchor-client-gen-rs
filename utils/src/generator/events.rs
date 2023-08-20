use std::str::FromStr;

use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    generator::{generate_fields, Module},
    idl::Idl,
};

pub fn generate(idl: &Idl) -> TokenStream {
    let mut generated = TokenStream::new();

    for event in &idl.events {
        let name = TokenStream::from_str(&event.name.to_pascal_case()).unwrap();
        let (fields, has_defined) = generate_fields(idl, &event.fields, Module::Events);
        let def = if has_defined {
            quote! {}
        } else {
            quote! {
                #[derive(Default)]
            }
        };

        generated.extend(quote! {
            #[event]
            #def
            pub struct #name {
                #fields
            }
        })
    }

    quote! {
        pub mod events {
            use anchor_lang::prelude::*;

            #generated
        }
    }
}
