use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr};

#[proc_macro]
pub fn hash_map(input: TokenStream) -> TokenStream {
    let entries = parse_macro_input!(input as Vec<(Expr, Expr)>);
    
    let entries = entries.iter().map(|(k, v)| quote! { (#k, #v) });

    let expanded = quote! {
        ::std::collections::HashMap::from([
            #(#entries),*
        ])
    };

    TokenStream::from(expanded)
}
