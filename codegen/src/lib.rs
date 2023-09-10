extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input,
    DeriveInput
};

#[proc_macro_derive(MetaType)]
pub fn metadata_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_name = &input.ident;

    (quote! {
        use suzaku_extension_sdk::meta::IMetaType;
        impl IMetaType for #type_name {
            fn get_name(&self) -> String {
                format!("{:?}", self)
            }
        }
    }).into()
}