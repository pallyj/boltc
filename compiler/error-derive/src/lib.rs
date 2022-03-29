use proc_macro::TokenStream;
use quote::quote;

extern crate proc_macro;

#[proc_macro_derive(Error)]
pub fn error(item: TokenStream) -> TokenStream {
    let item: syn::ItemEnum = syn::parse(item).unwrap();

    let mut variants = vec![];
    let mut indices = vec![];

    for variant in item.variants.iter().enumerate() {
        variants.push(variant.1.ident.clone());
        indices.push(variant.0);
    }

    let item_name = item.ident.clone();

    let tokens = quote! {
        impl #item_name {
            pub fn error_code(&self) -> String {
                match self {
                    #(<#item_name>::#variants => format!("E{}", #indices) ),*
                }
            }
        }
    };

    TokenStream::from(tokens)
}