use std::str::FromStr;

use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    generator::{generate_fields, Module},
    idl::IdlJsonDefinition,
    meta::Meta,
};

pub fn generate(idl: &IdlJsonDefinition, meta: &Meta) -> TokenStream {
    let mut generated = TokenStream::new();

    for event in &idl.events {
        let json_name = &event.name;
        let name = TokenStream::from_str(&json_name.to_pascal_case()).unwrap();
        let fields = generate_fields(idl, &event.fields, Module::Events, false);

        let copy_der = if meta.can_copy.get(json_name).is_some() {
            quote! {
                #[derive(Copy)]
            }
        } else {
            quote! {}
        };
        let def_der = if meta.can_default.get(json_name).is_some() {
            quote! {
                #[derive(Default)]
            }
        } else {
            quote! {}
        };

        generated.extend(quote! {
            #[event]
            #copy_der
            #def_der
            #[derive(Clone)]
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
