//! Derive proc-macro  definitions for the `Mock` trait.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, FieldsNamed};

/// Derives [`Mock`] for a struct if all of its fields implement `Mock`.
#[proc_macro_derive(Mock)]
pub fn derive_mock(token_stream: TokenStream) -> TokenStream {
    derive_mock_impl(token_stream)
}

fn derive_mock_impl(token_stream: TokenStream) -> TokenStream {
    let type_definition = parse_macro_input!(token_stream as DeriveInput);

    let identifier = type_definition.ident;

    match type_definition.data {
        Data::Struct(struct_data) => match struct_data.fields {
            Fields::Named(named_fields) => derive_struct_named(identifier, named_fields),
            Fields::Unnamed(tuple_fields) => derive_struct_tuple(identifier, tuple_fields),
            Fields::Unit => todo!(),
        },
        Data::Enum(_) => todo!(),
        Data::Union(_) => todo!(),
    }
}

fn derive_struct_named(identifier: syn::Ident, named_fields: FieldsNamed) -> TokenStream {
    let field_names = named_fields
        .named
        .into_iter()
        .map(|field| field.ident.expect("encountered named field without an identifier"));

    let fields = field_names.map(|field_name| quote! { #field_name: ::mock_default::Mock::mock() });

    quote! {
        impl ::mock_default::Mock for #identifier {
            fn mock() -> Self {
                Self {
                    #(#fields),*
                }
            }
        }
    }
    .into()
}

fn derive_struct_tuple(identifier: syn::Ident, tuple_fields: syn::FieldsUnnamed) -> TokenStream {
    let fields = tuple_fields.unnamed.iter().map(|_| quote! { ::mock_default::Mock::mock() });

    quote! {
        impl ::mock_default::Mock for #identifier {
            fn mock() -> Self {
                Self(#(#fields),*)
            }
        }
    }
    .into()
}
