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
                    impl actix_web::FromRequest for super::#ident {
                        type Error = actix_web::error::InternalError<&'static str>;
                        type Future = std::pin::Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

                        fn from_request(request: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
                            let variants_context = match request.app_data::<actix_web::web::Data<variants_actix_web::VaraintsContext>>() {
                                Some(context) => context,
                                None => {
                                            return Box::pin(async move {
                                                Err(actix_web::error::InternalError::new(
                                                    #error_msg,
                                                    actix_web::http::StatusCode::NOT_IMPLEMENTED))
                                                });
                                }
                            };

                            match variants_context.get_file(#file) {
                                Some(path) => {
                                    let mut variants = variants_actix_web::default::DefaultVariants::default();
                                    variants_context.build_varaints(request, &mut variants);
                                    let config_result =
                                        variants_actix_web::de::from_file_with_variants::<super::#ident, _, _>(path, &variants);
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
