//! See the documentation here [macron documentation](https://docs.rs/macron)

use proc_macro::TokenStream;
use quote::quote;

/// The implementation of trait Display
#[proc_macro_derive(Display, attributes(display))]
pub fn impl_display(input: TokenStream) -> TokenStream {
    let syn::DeriveInput { ident, data, attrs, .. } = syn::parse_macro_input!(input as syn::DeriveInput);
    
    match data {
        // impl Struct:
        syn::Data::Struct(st) => {
            // is has attr display:
            let output = if let Some(fmt) = read_attr_value(&attrs) {
                let args = st.fields
                    .iter()
                    .map(|field| field.ident.clone().unwrap())
                    .filter(|ident| {
                        let name = ident.to_string();
                        fmt.contains(&format!("{{{name}}}")) || fmt.contains(&format!("{{{name}:"))
                    });

                quote! { write!(f, #fmt, #(#args = &self.#args,)*) }
            }
            // no attr display:
            else {
                quote! { write!(f, stringify!(#ident)) }
            };

            quote! {
                impl ::std::fmt::Display for #ident {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        #output
                    }
                }
            }.into()
        },

        // impl Enum:
        syn::Data::Enum(en) => {
            if en.variants.is_empty() {
                return quote! {
                    impl ::std::fmt::Display for #ident {
                        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                            write!(f, "")
                        }
                    }
                }.into();
            }
            
            let vars = en.variants
                .iter()
                .map(|syn::Variant { ident: var_ident, fields, attrs, .. }| {
                    // is has attr display:
                    if let Some(mut fmt) = read_attr_value(&attrs) {
                        match fields {
                            // Named { ..:.., }
                            syn::Fields::Named(_) => {
                                let args = fields
                                    .iter()
                                    .map(|field| field.ident.clone().unwrap())
                                    .filter(|ident| {
                                        let name = ident.to_string();
    
                                        fmt.contains(&format!("{{{name}}}"))
                                        || fmt.contains(&format!("{{{name}:"))
                                    });
    
                                quote! { Self::#var_ident { #(#args,)* .. } => write!(f, #fmt) }
                            },
    
                            // Unnamed(.., .., )
                            syn::Fields::Unnamed(_) => {
                                let args = (0..fields.len())
                                    .into_iter()
                                    .map(|i| {
                                        let arg = if fmt.contains(&format!("{{{i}}}"))
                                        || fmt.contains(&format!("{{{i}:")) {
                                            format!("arg{i}")
                                        } else {
                                            fmt.push_str(&format!("{{{i}}}"));
                                            format!("_")
                                        };
                                        
                                        proc_macro2::Ident::new(&arg, proc_macro2::Span::call_site())
                                    })
                                    .collect::<Vec<_>>();
    
                                let vals = args.clone()
                                    .into_iter()
                                    .map(|arg| 
                                        if arg != "_" {
                                            quote! { #arg }
                                        } else {
                                            quote! { "" }
                                        }
                                    );
    
                                quote! { Self::#var_ident(#(#args,)*) => write!(f, #fmt, #(#vals,)*) }
                            },
    
                            _ => quote! { Self::#var_ident => write!(f, #fmt) }
                        }
                    }
                    // no attr display:
                    else {
                        quote! { Self::#var_ident => write!(f, stringify!(#var_ident)) }
                    }
                });
            
            quote! {
                impl ::std::fmt::Display for #ident {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        match &self {
                            #(
                                #vars,
                            )*
                        }
                    }
                }
            }.into()
        },

        _ => panic!("Expected a 'struct' or 'enum'")
    }
}

// Reads an attribute value
fn read_attr_value(attrs: &[syn::Attribute]) -> Option<String>{
    attrs
        .iter()
        .find(|attr| attr.path().is_ident("display"))
        .map(|attr| 
            match &attr.meta {
                syn::Meta::NameValue(meta) => {
                    if let syn::Expr::Lit(lit) = &meta.value {
                        if let syn::Lit::Str(s) = &lit.lit {
                            return s.value();
                        }
                    }

                    panic!("Expected the attribute format like this '#[display = \"a text..\"]'");
                },

                _ => panic!("Expected the attribute format like this '#[display = \"a text..\"]'")
            }
        )
}
