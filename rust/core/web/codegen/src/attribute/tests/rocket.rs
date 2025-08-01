#![cfg(test)]
use crate::attribute::rocket;
use quote::quote;

#[test]
fn test_vconfig_rocket_config() {
    let args = quote! {
        "test",
        file = "test"
    };

    let input = quote! {
        pub struct Test {
            key: u64,
        }
    };

    let expected = quote! {
        pub struct Test { key : u64 , }
        pub (crate) mod __test_impl___ {
            #[rocket::async_trait]
            impl <'r> rocket::request::FromRequest<'r> for super::Test {
                type Error = &'static str;
                async fn from_request(request: &'r rocket::Request < '_ >) -> rocket::request::Outcome<Self, Self::Error> {
                    let context = match request.rocket().state::<vconfig_rocket::VConfigContext>() {
                        Some(context) => context,
                        None => {
                                    return rocket::request::Outcome::Error((rocket::http::Status::InternalServerError, "Failed to deserialzie: Test"));
                        }
                    };

                    match context.get_file("test") {
                        Some (path) => {
                            let mut variants = vconfig_rocket::default::DefaultVariants::default();
                            context.build_variants(request, &mut variants);
                            let config_result = vconfig_rocket::de::from_file_with_variants::<super::Test, _ , _ >(path, &variants);
                            match config_result {
                                Ok(config) => rocket::request::Outcome::Success(config),
                                _ => rocket::request::Outcome::Error((rocket::http::Status::InternalServerError, "Failed to deserialzie: Test")),
                            }
                        }
                         _ => rocket::request::Outcome::Error((rocket::http::Status::InternalServerError, "Failed to deserialzie: Test")),
                    }
                }
            }
        }
    };
    let output = rocket::variant_config(args, input);
    assert_eq!(output.to_string(), expected.to_string());
}
