#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

/// Creates a new instance of PathBuf
#[proc_macro]
pub fn path(input: TokenStream) -> TokenStream {
    // empty string:
    if input.is_empty() {
        return quote! { ::std::path::PathBuf::new() }.into();
    }
    
    // string format:
    let Format { expr, args } = syn::parse_macro_input!(input as Format);
    
    let path_expr = match expr {
        syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(fmt), .. }) => {
            if args.is_some() || fmt.value().contains(&['{', '}'][..]) {
                quote! { ::std::path::PathBuf::from(::std::format!(#fmt #args)) }
            } else {
                quote! { ::std::path::PathBuf::from(#fmt) }
            }
        }
        _ => quote! { ::std::path::PathBuf::from(#expr) }
    };

    quote! {{
        let path = #path_expr;
        if path.to_string_lossy().starts_with("/") || path.to_string_lossy().starts_with("\\") {
            let exe_path = ::std::env::current_exe().expect("Failed to get exe path");
            let exe_dir = exe_path.parent().expect("Failed to get exe dir");

            let rel_str = path.to_str().expect("Invalid path");
            let rel_str = if rel_str.starts_with('/') || rel_str.starts_with('\\') {
                &rel_str[1..]
            } else {
                rel_str
            };
            exe_dir.join(rel_str)
        } else {
            path
        }
    }}.into()
}

/// The string formatter
struct Format {
    pub expr: syn::Expr,
    pub args: Option<TokenStream2>,
}

impl syn::parse::Parse for Format {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // parse expression:
        let expr = input.parse()?;

        // parse arguments:
        let args = if input.peek(syn::token::Comma) {
            Some(input.parse()?)
        } else {
            None
        };
        
        Ok(Self { expr, args })
    }
}
