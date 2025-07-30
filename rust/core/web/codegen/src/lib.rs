#![allow(dead_code)]
#[macro_use]
extern crate quote;

#[macro_use]
mod attribute;

#[doc(hidden)]
#[proc_macro_attribute]
pub fn actix_web_variants_config(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    if cfg!(feature = "actix_web") {
        attribute::actix_web::variants_config(args.into(), input.into()).into()
    } else {
        input
    }
}

#[doc(hidden)]
#[proc_macro_attribute]
pub fn axum_variants_config(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    if cfg!(feature = "axum") {
        attribute::axum::variants_config(args.into(), input.into()).into()
    } else {
        input
    }
}

#[doc(hidden)]
#[proc_macro_attribute]
pub fn rocket_variants_config(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    if cfg!(feature = "rocket") {
        attribute::rocket::variants_config(args.into(), input.into()).into()
    } else {
        input
    }
}
