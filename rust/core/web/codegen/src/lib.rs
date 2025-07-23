#![allow(unused_imports)]
#[macro_use]
extern crate quote;

#[macro_use]
mod attribute;

use proc_macro::TokenStream;

// todo: control with features.
#[doc(hidden)]
#[proc_macro_attribute]
pub fn config(args: TokenStream, input: TokenStream) -> TokenStream {
    attribute::rocket::config(args.into(), input.into()).into()
}
