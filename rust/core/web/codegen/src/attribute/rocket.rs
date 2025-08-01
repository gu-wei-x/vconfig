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
                    #[rocket::async_trait]
                    impl<'r> rocket::request::FromRequest<'r> for super::#ident {
                        type Error = &'static str;
                        async fn from_request(request: &'r rocket::Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
                            let context = match request.rocket().state::<vconfig_rocket::VariantsContext>() {
                                Some(context) => context,
                                None => {
                                            return rocket::request::Outcome::Error((rocket::http::Status::InternalServerError, #error_msg));
                                }
                            };

                            match context.get_file(#file) {
                                Some(path) => {
                                    let mut variants = vconfig_rocket::default::DefaultVariants::default();
                                    context.build_variants(request, &mut variants);
                                    let config_result =
                                        vconfig_rocket::de::from_file_with_variants::<super::#ident, _, _>(path, &variants);
                                    match config_result {
                                        Ok(config) => rocket::request::Outcome::Success(config),
                                        _ => rocket::request::Outcome::Error((rocket::http::Status::InternalServerError, #error_msg)),
                                    }
                                }
                                 _ => rocket::request::Outcome::Error((rocket::http::Status::InternalServerError, #error_msg)),
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
