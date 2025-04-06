//! See the documentation here [macron documentation](https://docs.rs/macron)

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

/// The implementation of trait Into
#[proc_macro_derive(Into, attributes(into))]
pub fn impl_into(input: TokenStream) -> TokenStream {
    let syn::DeriveInput { ident, data, attrs, .. } = syn::parse_macro_input!(input as syn::DeriveInput);
    
    let global_impls = read_attr_values(&attrs, None)
        .into_iter()
        .map(|AttrValue { ty, expr }| {
            quote! {
                impl ::std::convert::Into<#ty> for #ident {
                    fn into(self) -> #ty {
                        #expr
                    }
                }
            }
        });
    
    match data {
        // impl Struct:
        syn::Data::Struct(_st) => {
            quote! {
                #(
                    #global_impls
                )*
            }.into()
        },

        // impl Enum:
        syn::Data::Enum(_en) => {
            quote! {
                #(
                    #global_impls
                )*
            }.into()
        },

        _ => panic!("Expected a 'struct' or 'enum'")
    }
}

// Reads an attribute values
fn read_attr_values(attrs: &[syn::Attribute], fields: Option<&syn::Fields>) -> Vec<AttrValue> {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("into"))
        .map(|attr| {
            match &attr.meta {
                syn::Meta::List(list) => {
                    list
                        .parse_args()
                        .expect("Expected the attribute format like this '#[into(Type, \"a code..\")]'")
                },

                syn::Meta::Path(_) if fields.is_some() => {
                    let fields = fields.unwrap();
                    if fields.len() != 1 { panic!("Expected the one variant argument for the short attribute '#[into]'") }

                    let field = fields.iter().next().unwrap();
                    
                    AttrValue {
                        ty: field.ty.clone(),
                        expr: syn::parse_str( &if let Some(ident) = &field.ident { format!("self.{ident}") }else{ format!("arg0")} ).unwrap()
                    }
                },
                
                _ => panic!("Expected the attribute format like this '#[into(Type, \"a code..\")]'")
            }
        })
        .collect::<Vec<_>>()
}

struct AttrValue {
    pub ty: syn::Type,
    pub expr: TokenStream2
}

impl syn::parse::Parse for AttrValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ty: syn::Type = input.parse()?;
        
        input.parse::<syn::token::Comma>()?;
        
        let expr_s: syn::LitStr = input.parse()?;
        let expr: TokenStream2 = syn::parse_str(&expr_s.value())?;
        
        Ok(AttrValue {
            ty,
            expr,
        })
    }
}
