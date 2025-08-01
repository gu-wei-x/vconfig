#![cfg(test)]
use crate::attribute::axum;
use quote::quote;

#[test]
fn test_vconfig_axum_config() {
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
            impl<S> axum::extract::FromRequestParts<S> for super::Test
                    where
                        S: Send + Sync,
            {
                type Rejection = (axum::http::StatusCode, &'static str);

                async fn from_request_parts(parts: &mut axum::http::request::Parts,
                    _state: &S,) -> std::result::Result<Self, Self::Rejection> {
                    let variants_context = parts
                                .extensions
                                .get::<std::sync::Arc<vconfig_axum::VariantsContext>>()
                                .ok_or_else(|| {
                                    (
                                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                        "Failed to deserialzie: Test",
                                    )
                                })?;

                    match variants_context.get_file("test") {
                        Some(path) => {
                            let mut variants = vconfig_axum::default::DefaultVariants::default();
                            variants_context.build_variants(parts, &mut variants);
                            let config_result = vconfig_axum::de::from_file_with_variants::<super::Test, _, _>(
                                path,
                                &variants,
                            );
                            match config_result {
                                Ok(config) => Ok(config),
                                _ => Err((
                                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                        "Failed to deserialzie: Test",
                                    )),
                                }
                            }
                        _ => Err((
                                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                    "Failed to deserialzie: Test",
                                )),
                    }
                }
            }
        }
    };
    let output = axum::variants_config(args, input);
    assert_eq!(output.to_string(), expected.to_string());
}
