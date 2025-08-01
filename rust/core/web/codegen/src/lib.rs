#![allow(dead_code)]
#[macro_use]
extern crate quote;

#[macro_use]
mod attribute;

#[doc(hidden)]
#[proc_macro_attribute]
pub fn actix_web_variant_config(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    if cfg!(feature = "actix_web") {
        attribute::actix_web::variant_config(args.into(), input.into()).into()
    } else {
        input
    }
}

#[doc(hidden)]
#[proc_macro_attribute]
pub fn axum_variant_config(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    if cfg!(feature = "axum") {
        attribute::axum::variant_config(args.into(), input.into()).into()
    } else {
        input
    }
}

#[doc(hidden)]
#[proc_macro_attribute]
pub fn rocket_variant_config(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    if cfg!(feature = "rocket") {
        attribute::rocket::variant_config(args.into(), input.into()).into()
    } else {
        input
    }
}
