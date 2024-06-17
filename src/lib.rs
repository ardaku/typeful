//! #### A collection of helper derive macros for type patterns

#![doc(
    html_logo_url = "https://ardaku.github.io/mm/logo.svg",
    html_favicon_url = "https://ardaku.github.io/mm/icon.svg",
    html_root_url = "https://docs.rs/typeful"
)]
#![forbid(unsafe_code, missing_docs)]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]

mod common;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Error, Meta, Result};

fn impl_enum_functions(ast: &syn::DeriveInput) -> Result<TokenStream> {
    let name = &ast.ident;
    let attrs = &ast.attrs;

    for attr in attrs {
        let Meta::List(ref list) = attr.meta else {
            return Err(Error::new(
                attr.bracket_token.span.join(),
                "expected attribute list",
            ));
        };
        let path = &list.path;

        if path.leading_colon.is_some() {
            return Err(Error::new(
                list.delimiter.span().join(),
                "unexpected leading double colon",
            ));
        }

        if path.segments.len() != 1 {
            return Err(Error::new(
                list.delimiter.span().join(),
                "unexpected double colon in path",
            ));
        }

        let path = path.segments.first().unwrap();

        if !path.arguments.is_none() {
            return Err(Error::new(
                list.delimiter.span().join(),
                "unexpected path arguments",
            ));
        }

        if path.ident != "enum_functions" {
            return Err(Error::new(path.ident.span(), "unknown attribute"));
        }

        panic!("ya!");
    }

    let gen = quote! {
        impl #name {
        }
    };

    Ok(gen.into())
}

/// Derive a set of enum-related methods not tied to a trait.
#[proc_macro_derive(EnumFunctions, attributes(enum_functions))]
pub fn enum_functions_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    common::unwrap(impl_enum_functions(&ast).map(|ts| ts.into()))
}
