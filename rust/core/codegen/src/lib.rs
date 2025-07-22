#![allow(unused_extern_crates, unused_variables, unused_imports)]
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Fields, Type, parse_macro_input};

// todo: https://doc.rust-lang.org/reference/procedural-macros.html
#[proc_macro_attribute]
pub fn varaints_config(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{attr}\"");
    println!("item: \"{item}\"");
    item
}

// TODO:
// 1. must be on struct: item.data == syn::Data::Struct
// 2. attr contains file
// 3. output a generated impl, see rocket from FromRequest guard.
/*#[varaints_config(file="index")]
#[derive(Debug, Deserialize)]
#[serde(crate = "variantslib::serde")]
pub(crate) struct IndexConfig {
    pub(crate) welcome_msg: String,
}*/

/*#[rocket::async_trait]
impl<'r> FromRequest<'r> for IndexConfig {
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let configs = request
            .rocket()
            .state::<crate::variants::config::VaraintsConfig>()
            .unwrap();
        match configs.get_file("index") {
            Some(path) => {
                let mut variants_builder = crate::variants::builder::VariantsBuilder::default();
                variants_builder.config();
                let mut variants = DefaultVariants::default();
                variants_builder.build(request, &mut variants);
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
}*/

#[proc_macro_derive(Summarizable)]
pub fn summarize_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let generated = match &input.data {
        syn::Data::Struct(syn::DataStruct { fields, .. }) => match fields {
            Fields::Named(fields_named) => {
                let field_names: Vec<_> = fields_named.named.iter().map(|f| &f.ident).collect();
                let field_types: Vec<_> = fields_named.named.iter().map(|f| &f.ty).collect();

                quote! {
                    impl Summarizable for #name {
                        fn summary(&self) -> String {
                            #({
                                let field_value = &self.#field_names;
                                format!("{} : {}", stringify!(#field_names), field_value.to_string())
                            }),*
                        }
                    }
                }
            }
            _ => {
                quote! {}
            }
        },
        _ => {
            quote! {}
        }
    };

    TokenStream::from(generated)
}
