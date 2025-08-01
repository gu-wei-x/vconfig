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
                    impl actix_web::FromRequest for super::#ident {
                        type Error = actix_web::error::InternalError<&'static str>;
                        type Future = std::pin::Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

                        fn from_request(request: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
                            let vconfig_context = match request.app_data::<actix_web::web::Data<vconfig_actix_web::VConfigContext>>() {
                                Some(context) => context,
                                None => {
                                            return Box::pin(async move {
                                                Err(actix_web::error::InternalError::new(
                                                    #error_msg,
                                                    actix_web::http::StatusCode::NOT_IMPLEMENTED))
                                                });
                                }
                            };

                            match vconfig_context.get_file(#file) {
                                Some(path) => {
                                    let mut variants = vconfig_actix_web::DefaultVariants::default();
                                    vconfig_context.build_variants(request, &mut variants);
                                    let config_result =
                                        vconfig_actix_web::de_from_file::<super::#ident, _, _>(path, &variants);
                                        match config_result {
                                            Ok(config) => Box::pin(async move { Ok(config) }),
                                            _ => Box::pin(async move {
                                                Err(actix_web::error::InternalError::new(
                                                    #error_msg,
                                                    actix_web::http::StatusCode::NOT_IMPLEMENTED))
                                            }),
                                        }
                                }
                                _ => Box::pin(async move {
                                         Err(actix_web::error::InternalError::new(
                                             #error_msg,
                                             actix_web::http::StatusCode::NOT_IMPLEMENTED))
                                }),
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
