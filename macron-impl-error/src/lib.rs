//! See the documentation here [macron documentation](https://docs.rs/macron)

use proc_macro::TokenStream;
use quote::quote;

/// The implementation of trait Error
#[proc_macro_derive(Error, attributes(source))]
pub fn impl_error(input: TokenStream) -> TokenStream {
    let syn::DeriveInput { ident, data, .. } = syn::parse_macro_input!(input as syn::DeriveInput);
    
    match data {
        // impl Struct:
        syn::Data::Struct(st) => {
            let source = match st.fields {
                syn::Fields::Named(fields) => {
                    let src_field = fields.named
                        .iter()
                        .find(|f| f.ident
                            .as_ref()
                            .map(|i| i == "source")
                            .unwrap_or(false)
                        );
            
                    match src_field {
                        Some(field) if is_option_type(&field.ty) => quote! { self.source.as_deref() },

                        Some(_) => quote! { Some(self.source) },

                        None => quote! { None }
                    }
                },

                _ => quote! { None }
            };
            
            quote! {
                impl ::std::error::Error for #ident {
                    fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
                        #source
                    }
                }
            }.into()
        },

        // impl Enum:
        syn::Data::Enum(en) => {
            let vars = en.variants
                .into_iter()
                .map(|syn::Variant { ident: var_ident, fields, .. }| {
                    match fields {
                        // Fields::Named { ..: .. }
                        syn::Fields::Named(fields) => {
                            let src_field = fields.named
                                .iter()
                                .find(|f| f.ident
                                    .as_ref()
                                    .map(|i| i == "source")
                                    .unwrap_or(false)
                                );
                    
                            match src_field {
                                Some(field) if is_option_type(&field.ty) => quote! { Self::#var_ident { source, .. } => source.as_deref() },

                                Some(_) => quote! { Self::#var_ident { source, .. } => Some(source) },

                                None => quote! { Self::#var_ident(_) => None }
                            }
                        },

                        // Fields::Unnamed(.., ..)
                        syn::Fields::Unnamed(fields) => {
                            let src_field = fields.unnamed
                                .iter()
                                .enumerate()
                                .find(|(_, field)| field.attrs
                                    .iter()
                                    .any(|attr| attr.path().is_ident("source"))
                                );

                            let src_idx = src_field.map(|(idx, _)| idx).unwrap_or(0);
                            let stubs = (0..src_idx).into_iter().map(|_| quote! { _ });

                            match src_field {
                                Some((_, field)) if is_option_type(&field.ty) => quote! { Self::#var_ident(#(#stubs,)* source) => source.as_deref() },

                                Some(_) => quote! { Self::#var_ident(#(#stubs,)* source, ..) => Some(source) },

                                None => quote! { Self::#var_ident(_) => None }
                            }
                        },

                        // Self::Unit
                        syn::Fields::Unit => quote! { Self::#var_ident => None }
                    }
                });
            
            quote! {
                impl ::std::error::Error for #ident {
                    fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
                        match &self {
                            #(
                                #vars,
                            )*
                        }
                    }
                }
            }.into()
        },

        _ => panic!("the expected a 'struct' or 'enum'")
    }
}

// Do check if a type is "Option"
fn is_option_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        type_path.path.segments.last().map_or(false, |seg| seg.ident == "Option")
    } else {
        false
    }
}
