use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// Macro de derivação para implementar automaticamente `MeuTrait`
#[proc_macro_derive(MeuTrait)]
pub fn meu_trait_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let expanded = quote! {
        impl MeuTrait for #name {
            fn minha_funcao(&self) -> String {
                String::from("Implementação padrão de MeuTrait")
            }
        }
    };

    TokenStream::from(expanded)
}
