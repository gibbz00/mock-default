//! Derive proc-macro  definitions for the `Mock` trait.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, FieldsNamed};

/// Derives [`Mock`] for a struct if all of its fields implement `Mock`.
#[proc_macro_derive(Mock, attributes(mock))]
pub fn derive_mock(token_stream: TokenStream) -> TokenStream {
    derive_mock_impl(token_stream)
}

fn derive_mock_impl(token_stream: TokenStream) -> TokenStream {
    let type_definition = parse_macro_input!(token_stream as DeriveInput);

    let identifier = type_definition.ident;

    let self_definition_result = match type_definition.data {
        Data::Struct(data_struct) => derive_struct(data_struct),
        Data::Enum(data_enum) => derive_enum(data_enum),
        Data::Union(_) => todo!(),
    };

    match self_definition_result {
        Ok(self_definition) => {
            quote! {
                impl ::mock_default::Mock for #identifier {
                    fn mock() -> Self {
                        #self_definition
                    }
                }
            }
        }
        Err(err) => err.to_compile_error(),
    }
    .into()
}

mod fields {
    use super::*;

    pub fn named(named_fields: FieldsNamed) -> impl Iterator<Item = proc_macro2::TokenStream> {
        let field_names = named_fields
            .named
            .into_iter()
            .map(|field| field.ident.expect("encountered named field without an identifier"));

        field_names.map(|field_name| quote! { #field_name: ::mock_default::Mock::mock() })
    }

    pub fn tuple(tuple_fields: syn::FieldsUnnamed) -> impl Iterator<Item = proc_macro2::TokenStream> {
        tuple_fields.unnamed.into_iter().map(|_| quote! { ::mock_default::Mock::mock() })
    }
}

fn derive_struct(data_struct: syn::DataStruct) -> syn::Result<proc_macro2::TokenStream> {
    Ok(match data_struct.fields {
        Fields::Named(named_fields) => {
            let fields = fields::named(named_fields);

            quote! {
                Self {
                    #(#fields),*
                }
            }
        }
        Fields::Unnamed(tuple_fields) => {
            let fields = fields::tuple(tuple_fields);

            quote! { Self(#(#fields),*) }
        }
        Fields::Unit => quote! { Self },
    })
}

fn derive_enum(data_enum: syn::DataEnum) -> syn::Result<proc_macro2::TokenStream> {
    let mut variant_to_mock_iter = data_enum.variants.into_iter().filter_map(|variant| {
        variant
            .attrs
            .clone()
            .iter()
            .find(|attribute| match &attribute.meta {
                syn::Meta::Path(path) => path.get_ident().is_some_and(|ident| &ident.to_string() == "mock"),
                _ => false,
            })
            .map(|_| variant)
    });

    let Some(variant_to_mock) = variant_to_mock_iter.next() else {
        return Err(syn::Error::new(
            data_enum.enum_token.span,
            "no #[mock] attribute found in any of the listed variants",
        ));
    };

    if let Some(_another_variant_to_mock) = variant_to_mock_iter.next() {
        return Err(syn::Error::new(
            data_enum.enum_token.span,
            "expected only one #[mock] enum variant attribute, unable to infer which one to use.",
        ));
    }

    let variant_name = variant_to_mock.ident;

    Ok(match variant_to_mock.fields {
        Fields::Named(named_fields) => {
            let fields = fields::named(named_fields);

            quote! {
                Self::#variant_name {
                    #(#fields),*
                }
            }
        }
        Fields::Unnamed(tuple_fields) => {
            let fields = fields::tuple(tuple_fields);
            quote! {
                Self::#variant_name(#(#fields),*)
            }
        }
        Fields::Unit => {
            quote! {
                Self::#variant_name
            }
        }
    })
}
