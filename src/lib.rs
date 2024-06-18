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
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Data, Error, Ident, Meta, Result, Token,
};

struct AttrParams {
    attrs: Punctuated<Ident, Token![,]>,
}

impl Parse for AttrParams {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            attrs: Punctuated::parse_terminated(input)?,
        })
    }
}

fn impl_enum_functions(
    ast: syn::DeriveInput,
) -> Result<proc_macro2::TokenStream> {
    let mut token_stream = proc_macro2::TokenStream::new();
    let name = ast.ident;
    let attrs = ast.attrs;
    let data = ast.data;

    for attr in attrs {
        let Meta::List(list) = attr.meta else {
            continue;
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

        let Data::Enum(ref data_enum) = data else {
            return Err(Error::new(path.ident.span(), "expected enum"));
        };
        let params: AttrParams = syn::parse2(list.tokens)?;
        let variants: Punctuated<syn::Path, Token![,]> = data_enum
            .variants
            .iter()
            .map(|v| -> syn::Path {
                let ident = &v.ident;

                syn::parse_quote!(Self::#ident)
            })
            .collect();
        let variant_count =
            proc_macro2::Literal::usize_suffixed(variants.len());
        let mut has_variant_array = false;
        let mut has_variant_count = false;

        for attr in params.attrs {
            match attr {
                a if a == "variant_array" => {
                    if has_variant_array {
                        return Err(Error::new(
                            a.span(),
                            "duplicated attribute",
                        ));
                    }

                    has_variant_array = true;
                    token_stream.extend(quote! {
                        impl #name {
                            const fn variant_array<const N: usize>() -> [Self; N] {
                                if N > #variant_count {
                                    panic!("requested variant array is too large")
                                }

                                let full_array = [#variants];
                                let mut array = [full_array[0]; N];
                                let mut i = 1;

                                while i < N {
                                    array[i] = full_array[i];
                                    i += 1;
                                }

                                array
                            }
                        }
                    });
                }
                a if a == "variant_count" => {
                    if has_variant_count {
                        return Err(Error::new(
                            a.span(),
                            "duplicated attribute",
                        ));
                    }

                    has_variant_count = true;
                    token_stream.extend(quote! {
                        impl #name {
                            const fn variant_count() -> usize {
                                #variant_count
                            }
                        }
                    });
                }
                ident => {
                    return Err(Error::new(ident.span(), "unknown attribute"))
                }
            }
        }
    }

    Ok(token_stream)
}

/// Derive a set of enum-related methods not tied to a trait.
#[proc_macro_derive(EnumFunctions, attributes(enum_functions))]
pub fn enum_functions_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    common::unwrap(impl_enum_functions(ast))
}
