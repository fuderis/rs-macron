//! See the documentation here [macron documentation](https://docs.rs/macron)

use proc_macro::TokenStream;
use quote::quote;
use syn::{ punctuated::Punctuated, spanned::Spanned };

/// Creates a new instance of [String](https://doc.rust-lang.org/stable/std/string/struct.String.html)
#[proc_macro]
pub fn str(input: TokenStream) -> TokenStream {
    let Format { expr, args } = syn::parse_macro_input!(input as Format);
    
    match expr {
        // literal string:
        syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(fmt), .. }) => {
            // str!("a format {}..", "text")
            if args.is_some() {
                let args = args.unwrap().into_iter();
                
                quote! { ::std::format!(#fmt, #(#args)*) }
            }
            // str!("a format {text}..")
            else if fmt.value().contains(&['{', '}'][..])  {
                quote! { ::std::format!(#fmt) }
            }
            // str!("a simple text..")
            else {
                quote! { #fmt.to_owned() }
            }
        },

        _ => quote! { #expr.to_string() }
    }.into()
}

// The string formatter
struct Format {
    pub expr: syn::Expr,
    pub args: Option<Punctuated::<syn::Expr, syn::Token![,]>>,
}

impl syn::parse::Parse for Format {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let expr = input.parse()?;

        let args = match input.parse::<syn::Token![,]>() {
            Ok(_) => {
                if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(_), .. }) = expr {
                } else {
                    return Err(syn::Error::new(expr.span(), "Expected the literal string by using format arguments, example: 'str!(\"format str {{}}\", \"arg1\", ..)'"));
                }
                
                Some(Punctuated::parse_terminated(input)?)
            }, 
            Err(_) => None
        };
        
        Ok(Self { expr, args })
    }
}
