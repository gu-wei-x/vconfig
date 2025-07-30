use devise::{FromMeta, MetaItem};
use proc_macro2::TokenStream;
use proc_macro2::{Ident, Span};
use syn::Data;
use syn::DeriveInput;

#[derive(Debug, FromMeta)]
struct Config {
    #[meta(naked)]
    pub name: Option<String>,
    pub file: Option<String>,
}

pub(crate) fn variants_config(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_tokens = quote!(config(#args));
    let att_meta = &syn::parse2::<MetaItem>(attr_tokens).unwrap();
    let attribute = Config::from_meta(att_meta).unwrap();

    let ast: DeriveInput = syn::parse2(input).unwrap();
    match ast.data {
        Data::Struct(_) => {
            let ident = &ast.ident;
            let mod_name = format!("__{}_impl___", ident.to_string().to_lowercase());
            let mode_ident = Ident::new(&mod_name, Span::call_site());
            let error_msg = format!("Failed to deserialzie: {}", ident.to_string());
            let file = match attribute.file {
                Some(path) => path,
                _ => attribute.name.unwrap(),
            };

            let implemtation = quote! {
                pub(crate) mod #mode_ident {
                    impl<S> axum::extract::FromRequestParts<S> for super::#ident
                    where
                        S: Send + Sync,
                    {
                        type Rejection = (axum::http::StatusCode, &'static str);

                        async fn from_request_parts(
                            parts: &mut axum::http::request::Parts,
                            _state: &S,
                        ) -> std::result::Result<Self, Self::Rejection> {
                            let variants_context = parts
                                .extensions
                                .get::<std::sync::Arc<variants_axum::VariantsContext>>()
                                .ok_or_else(|| {
                                    (
                                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                        #error_msg,
                                    )
                                })?;

                            match variants_context.get_file(#file) {
                                Some(path) => {
                                    let mut variants = variants_axum::default::DefaultVariants::default();
                                    variants_context.build_variants(parts, &mut variants);
                                    let config_result = variants_axum::de::from_file_with_variants::<super::#ident, _, _>(
                                        path,
                                        &variants,
                                    );
                                    match config_result {
                                        Ok(config) => Ok(config),
                                        _ => Err((
                                            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                            #error_msg,
                                        )),
                                    }
                                }
                                _ => Err((
                                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                    #error_msg,
                                )),
                            }
                        }
                    }
                }
            };

            quote! {
                #ast
                #implemtation
            }
        }
        _ => {
            // not supported for other types
            quote! {
                #ast
            }
        }
    }
}
