//! Derives the [`ActionKey`] trait
//
//! This derive macro was inspired by the `strum` crate's `EnumIter` macro.
//! Original source: https://github.com/Peternator7/strum,
//! Copyright (c) 2019 Peter Glotfelty under the MIT License

extern crate proc_macro;
mod action_key;
use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(ActionKey)]
pub fn action_key(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    crate::action_key::action_key_inner(&ast).into()
}
