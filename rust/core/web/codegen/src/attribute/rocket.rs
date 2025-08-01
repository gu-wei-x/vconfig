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
                    #[rocket::async_trait]
                    impl<'r> rocket::request::FromRequest<'r> for super::#ident {
                        type Error = &'static str;
                        async fn from_request(request: &'r rocket::Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
                            let context = match request.rocket().state::<vconfig_rocket::VConfigContext>() {
                                Some(context) => context,
                                None => {
                                            return rocket::request::Outcome::Error((rocket::http::Status::InternalServerError, #error_msg));
                                }
                            };

                            match context.get_file(#file) {
                                Some(path) => {
                                    let mut variants = vconfig_rocket::DefaultVariants::default();
                                    context.build_variants(request, &mut variants);
                                    let config_result =
                                        vconfig_rocket::de_from_file::<super::#ident, _, _>(path, &variants);
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
