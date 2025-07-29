#![cfg(test)]
use crate::attribute::actix_web;
use quote::quote;

#[test]
fn test_variants_actix_web_config() {
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
            impl actix_web::FromRequest for super::Test {
                type Error = actix_web::error::InternalError<&'static str>;
                type Future = std::pin::Pin<Box<dyn Future<Output = Result<Self , Self::Error>>>>;

                fn from_request(request: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
                    let variants_context = match request.app_data::<actix_web::web::Data<variants_actix_web::VariantsContext>>() {
                        Some(context) => context,
                        None => {
                                    return Box::pin(async move {
                                                Err(actix_web::error::InternalError::new(
                                                    "Failed to deserialzie: Test",
                                                    actix_web::http::StatusCode::NOT_IMPLEMENTED))
                                                });
                        }
                    };

                    match variants_context.get_file("test") {
                        Some(path) => {
                            let mut variants = variants_actix_web::default::DefaultVariants::default();
                            variants_context.build_variants(request , &mut variants);
                            let config_result = variants_actix_web::de::from_file_with_variants::<super::Test , _ , _>(path, &variants);
                            match config_result {
                                Ok(config) => Box::pin(async move { Ok(config) }),
                                _ => Box::pin(async move {
                                                Err(actix_web::error::InternalError::new("Failed to deserialzie: Test", actix_web::http::StatusCode::NOT_IMPLEMENTED))
                                              }),
                            }
                        }
                        _ => Box::pin(async move {
                                                Err(actix_web::error::InternalError::new("Failed to deserialzie: Test", actix_web::http::StatusCode::NOT_IMPLEMENTED))
                                              }),
                    }
                }
            }
        }
    };
    let output = actix_web::variants_config(args, input);
    assert_eq!(output.to_string(), expected.to_string());
}
