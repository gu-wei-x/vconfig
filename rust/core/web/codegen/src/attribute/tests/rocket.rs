#![cfg(test)]
use crate::attribute::rocket;
use quote::quote;

#[test]
fn test_variants_rocket_config() {
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
        pub (crate) mod test_impl___ {
            extern crate variants as variantslib ;
            #[rocket::async_trait]
            impl <'r> rocket::request::FromRequest<'r> for super::Test {
                type Error = () ;
                async fn from_request (request: &'r rocket::Request < '_ >) -> rocket::request::Outcome<Self, Self::Error> {
                    let context = request.rocket().state::<crate::variants::config::VaraintsContext>().unwrap();
                    match context.get_file ("test") {
                        Some (path) => {
                            let mut variants = variantslib::default::DefaultVariants::default();
                            context.build_varaints(request, &mut variants);
                            let config_result = variantslib::de::from_file_with_variants::<super::Test, _ , _ >(path, &variants);
                            match config_result {
                                Ok(config) => rocket::request::Outcome::Success(config),
                                _ => rocket::request::Outcome::Forward(rocket::http::Status{ code : 500 }),
                            }
                        }
                         _ => rocket::request::Outcome::Forward(rocket::http::Status{ code : 500 }),
                        }
                    }
                }
            }
    };
    let output = rocket::config(args, input);
    assert_eq!(output.to_string(), expected.to_string());
}
