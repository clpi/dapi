extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Test)]
pub fn derive_fn(input: proc_macro::TokenStream) -> TokenStream {
    input
}

