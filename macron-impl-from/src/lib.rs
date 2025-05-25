#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

//! See the documentation here [macron documentation](https://docs.rs/macron)

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

/// The implementation of trait From
#[proc_macro_derive(From, attributes(from))]
pub fn impl_from(input: TokenStream) -> TokenStream {
    let syn::DeriveInput { ident, data, attrs, .. } = syn::parse_macro_input!(input as syn::DeriveInput);
    
    let global_impls = read_attr_values(&attrs, None)
        .into_iter()
        .map(|AttrValue { ty, expr }| {
            quote! {
                impl ::std::convert::From<#ty> for #ident {
                    fn from(value: #ty) -> Self {
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
        syn::Data::Enum(en) => {
            let var_impls = en.variants
                .iter()
                .map(|syn::Variant { ident: var_ident, attrs, fields, .. }| {
                    let var_impls = read_attr_values(&attrs, Some(&fields))
                        .into_iter()
                        .map(|AttrValue { ty, expr }| {
                            let output = match &fields {
                                syn::Fields::Named(_) => quote! { Self::#var_ident { #expr } },
                                syn::Fields::Unnamed(_) => quote! { Self::#var_ident(#expr) },
                                syn::Fields::Unit => quote! { Self::#var_ident }
                            };
                            
                            quote! {
                                impl ::std::convert::From<#ty> for #ident {
                                    fn from(value: #ty) -> Self {
                                        #output
                                    }
                                }
                            }
                        });

                    quote! { #(#var_impls)* }
                });
            
            quote! {
                #(
                    #global_impls
                )*

                #(
                    #var_impls
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
        .filter(|attr| attr.path().is_ident("from"))
        .map(|attr| {
            match &attr.meta {
                syn::Meta::List(list) => {
                    list
                        .parse_args()
                        .expect("Expected the attribute format like this '#[from(Type, \"a code..\")]'")
                },

                syn::Meta::Path(_) if fields.is_some() => {
                    let fields = fields.unwrap();
                    if fields.len() != 1 { panic!("Expected the one variant argument for the short attribute '#[from]'") }

                    let field = fields.iter().next().unwrap();
                    
                    AttrValue {
                        ty: field.ty.clone(),
                        expr: syn::parse_str( &if let Some(ident) = &field.ident { format!("{ident}: value") }else{ format!("value")} ).unwrap()
                    }
                },
                
                _ => panic!("Expected the attribute format like this '#[from(Type, \"a code..\")]'")
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
