#![allow(unused_imports)]
use devise::{FromMeta, MetaItem};
use proc_macro2::TokenStream;
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
            let file = match attribute.file {
                Some(path) => path,
                _ => attribute.name.unwrap(),
            };

            /*let raw_str = format!(
                r#"const _: () = {{
extern crate variants as variantslib;
use rocket::Request;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use variantslib::default::DefaultVariants;
use variantslib::serde::Deserialize;
#[rocket::async_trait]
impl<'r> FromRequest<'r> for {} {{
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {{
        let configs = request
            .rocket()
            .state::<crate::variants::config::VaraintsConfig>()
            .unwrap();
        match configs.get_file("{}") {{
            Some(path) => {{
                let mut variants = DefaultVariants::default();
                configs.build_varaints(request, &mut variants);
                let config_result =
                    variantslib::de::from_file_with_variants::<IndexConfig, _, _>(path, &variants);
                match config_result {{
                    Ok(config) => Success(config),
                    _ => Forward(rocket::http::Status {{ code: 500 }}),
                }}
            }}
            _ => Forward(rocket::http::Status {{ code: 500 }}),
        }}
    }}
            }}}}"#,
                ident.to_string(),
                file
            );
            println!("{}", raw_str);*/
            quote! {
                #ast
                //#raw_str
            /*const _: () = {
            extern crate variants as variantslib;
            use rocket::Request;
            use rocket::request::{FromRequest, Outcome};
            use variantslib::config;
            use variantslib::default::DefaultVariants;
            use variantslib::serde::Deserialize;*/

            #[rocket::async_trait]
            impl<'r> FromRequest<'r> for #ident {
                type Error = ();
                async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
                    let configs = request
                        .rocket()
                        .state::<crate::variants::config::VaraintsConfig>()
                        .unwrap();
                    match configs.get_file(#file) {
                        Some(path) => {
                            let mut variants = DefaultVariants::default();
                            configs.build_varaints(request, &mut variants);
                            let config_result =
                                variantslib::de::from_file_with_variants::<IndexConfig, _, _>(path, &variants);
                            match config_result {
                                Ok(config) => Outcome::Success(config),
                                _ => Outcome::Forward(rocket::http::Status { code: 500 }),
                            }
                        }
                        _ => Outcome::Forward(rocket::http::Status { code: 500 }),
                    }
                }
            }}

            //}
        }
        _ => {
            // not supported on other types
            quote! {
                #ast
            }
        }
    }
}
