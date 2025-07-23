/*#![cfg(test)]
#![allow(unused_imports)]
use crate::attribute::rocket;
use quote::quote;

#[test]
fn test_variants_rocket_config() {
    let args = quote! {
        #[config(name = "")]
    };

    let input = quote! {
        pub struct Test {
            key: u64,
        }
    };

    let expected = quote! {
        test;
    };

    let output = rocket::config(args, input);
    assert_eq!(output.to_string(), expected.to_string());
}*/
