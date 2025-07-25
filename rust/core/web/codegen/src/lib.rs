#![allow(dead_code)]
#[macro_use]
extern crate quote;

#[macro_use]
mod attribute;

#[cfg(feature = "actix_web")]
#[doc(hidden)]
#[proc_macro_attribute]
pub fn variants_config(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    attribute::actix_web::variants_config(args.into(), input.into()).into()
}

#[cfg(feature = "rocket")]
#[doc(hidden)]
#[proc_macro_attribute]
pub fn variants_config(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    attribute::rocket::variants_config(args.into(), input.into()).into()
}
