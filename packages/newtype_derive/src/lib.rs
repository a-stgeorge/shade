use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::{Data, DataStruct, Fields};

#[proc_macro_derive(internal_forward_newtype)]
pub fn newtype_macro_derive_internal(input: TokenStream) -> TokenStream {
    // Build syntax tree
    let ast = syn::parse(input).unwrap();

    impl_newtype_macro(ast,true)
}

#[proc_macro_derive(forward_newtype)]
pub fn newtype_macro_derive(input: TokenStream) -> TokenStream {
    // Build syntax tree
    let ast = syn::parse(input).unwrap();

    impl_newtype_macro(ast, false)
}

fn impl_newtype_macro(ast: syn::DeriveInput, internal: bool) -> TokenStream {
    let name = ast.ident;

    let fields_punct = match ast.data {
        Data::Struct(
            DataStruct {
                fields: Fields::Unnamed(fields),
                ..
            }
        ) => fields.unnamed,
        _ => panic!("")
    };

    let newtype_type = match fields_punct.first() {
        None => panic!("Empty structs are not allowed"),
        Some(field) => field.ty.clone()
    };

    let import = match internal {
        true => quote!(use crate::utils::storage::ForwardNewtype;),
        false => quote!(use shade_protocol::utils::storage::ForwardNewtype;)
    };

    TokenStream::from(
        quote!(
            #import

            impl ForwardNewtype<#newtype_type> for #name {
                fn item(&self) -> #newtype_type {
                    self.0
                }

                fn item_ref(&self) -> &#newtype_type {
                    &self.0
                }

                fn item_mut(&mut self) -> &mut #newtype_type {
                    &mut self.0
                }

                fn item_set(&mut self, item: #newtype_type) {
                    self.0 = item;
                }
            }

            impl From<#newtype_type> for #name {
                fn from(item: #newtype_type) -> Self {
                    Self(item)
                }
            }
        )
    )
}