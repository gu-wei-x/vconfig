use devise::{FromMeta, MetaItem};
use proc_macro2::TokenStream;
use proc_macro2::{Ident, Span};
use std::fmt::format;
use syn::{Attribute, Data};
use syn::{DeriveInput, Fields, Type, parse_macro_input};

#[derive(Debug, FromMeta)]
pub struct Config {
    #[meta(naked)]
    pub name: Option<String>,
    pub file: Option<String>,
}

pub(crate) fn config(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_tokens = quote!(config(#args));
    let att_meta = &syn::parse2::<MetaItem>(attr_tokens).unwrap();
    let attribute = Config::from_meta(att_meta).unwrap();

    let ast: DeriveInput = syn::parse2(input).unwrap();
    match ast.data {
        Data::Struct(_) => {
            let ident = &ast.ident;
            let mod_name = format!("{}_impl___", ident.to_string().to_lowercase());
            let mode_ident = Ident::new(&mod_name, Span::call_site());
            let file = match attribute.file {
                Some(path) => path,
                _ => attribute.name.unwrap(),
            };

            let implemtation = quote! {
                pub(crate) mod #mode_ident {
                    extern crate variants as variantslib;
                    #[rocket::async_trait]
                    impl<'r> rocket::request::FromRequest<'r> for super::#ident {
                        type Error = ();
                        async fn from_request(request: &'r rocket::Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
                            let context = request.rocket().state::<crate::variants::config::VaraintsContext>().unwrap();
                            match context.get_file(#file) {
                                Some(path) => {
                                    let mut variants = variantslib::default::DefaultVariants::default();
                                    context.build_varaints(request, &mut variants);
                                    let config_result =
                                        variantslib::de::from_file_with_variants::<super::#ident, _, _>(path, &variants);
                                    match config_result {
                                        Ok(config) => rocket::request::Outcome::Success(config),
                                        _ => rocket::request::Outcome::Forward(rocket::http::Status { code: 500 }),
                                    }
                                }
                                 _ => rocket::request::Outcome::Forward(rocket::http::Status { code: 500 }),
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
            // not supported on other types
            quote! {
                #ast
            }
        }
    }
}
