use proc_macro2::TokenStream;
use proc_macro2::{Ident, Span};
use syn::Data;
use syn::DeriveInput;

struct Config {
    pub name: Option<String>,
    pub file: Option<String>,
}

pub(crate) fn variant_config(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_tokens = quote!(config(#args));
    let (name, path) = crate::extract_file_path(attr_tokens.clone().into());
    let attribute = Config {
        name: name,
        file: path,
    };

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
                            let vconfig_context = parts
                                .extensions
                                .get::<std::sync::Arc<vconfig_axum::VConfigContext>>()
                                .ok_or_else(|| {
                                    (
                                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                        #error_msg,
                                    )
                                })?;

                            match vconfig_context.get_file(#file) {
                                Some(path) => {
                                    let mut variants = vconfig_axum::DefaultVariants::default();
                                    vconfig_context.build_variants(parts, &mut variants);
                                    let config_result = vconfig_axum::de_from_file::<super::#ident, _, _>(
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
