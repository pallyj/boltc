use proc_macro::TokenStream;
use quote::quote;

extern crate proc_macro;

#[proc_macro_derive(Error)]
pub fn error(item: TokenStream) -> TokenStream {
    let item: syn::ItemEnum = syn::parse(item).unwrap();

    let item_name = item.ident.clone();

    let error_code_branches = item.variants.iter().enumerate().map(|variant| {
                                                                  let variant_name = &variant.1.ident;
                                                                  let indice = variant.0;
                                                                  quote! { #item_name::#variant_name { .. } => format!("E{:03}", #indice) }
                                                              });

    let tokens = quote! {
        impl #item_name {
            pub fn error_code(&self) -> String {
                match self {
                    #(#error_code_branches),*
                }
            }
        }
    };

    TokenStream::from(tokens)
}
