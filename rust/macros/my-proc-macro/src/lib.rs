extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Hello)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream
{
   let input = parse_macro_input!(input as DeriveInput);
   let name = &input.ident;

   let expanded = quote! {
       impl #name {
           pub fn say_hello(&self) {
               println!("Hello from {}!", stringify!(#name));
           }
       }
   };

   TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn log_call(_attr: TokenStream, item: TokenStream) -> TokenStream
{
   let input = syn::parse_macro_input!(item as syn::ItemFn);
   let name = &input.sig.ident;

   quote! {
       #input
       fn #name() {
           println!("Calling {}", stringify!(#name));
           #name()
       }
   }
   .into()
}
