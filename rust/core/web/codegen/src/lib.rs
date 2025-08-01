#![allow(dead_code)]
#[macro_use]
extern crate quote;

#[macro_use]
mod attribute;

/// Attribute to generate an [`actix_web::FromRequest`] implementation for the struct applied to.
///
/// This attribute can only be applied to structs.
///
/// # Grammar
///
/// The grammar for the `#[actix_web_variant_config]` attributes is defined as:
///
/// ```text
/// actix_web_variant_config := Name* | File=Path
///
/// Name := configuration file name without extension, e.g. `actix_web_variant_config("test")`
/// Path := path to the configuration file, e.g. `actix_web_variant_config(file = "test.toml")`
/// ```
///
/// [`actix_web::FromRequest`]: https://docs.rs/actix-web/latest/actix_web/trait.FromRequest.html
#[proc_macro_attribute]
pub fn actix_web_variant_config(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    if cfg!(feature = "actix_web") {
        attribute::actix_web::variant_config(args.into(), input.into()).into()
    } else {
        input
    }
}

/// Attribute to generate an [`axum::extract::FromRequestParts<S>`] implementation for the struct applied to.
///
/// This attribute can only be applied to structs.
///
/// # Grammar
///
/// The grammar for the `#[axum_variant_config]` attributes is defined as:
///
/// ```text
/// axum_variant_config := Name* | File=Path
///
/// Name := configuration file name without extension, e.g. `axum_variant_config("test")`
/// Path := path to the configuration file, e.g. `axum_variant_config(file = "test.toml")`
/// ```
///
/// [`axum::extract::FromRequestParts<S>`]: https://docs.rs/axum/latest/axum/extract/trait.FromRequestParts.html
#[proc_macro_attribute]
pub fn axum_variant_config(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    if cfg!(feature = "axum") {
        attribute::axum::variant_config(args.into(), input.into()).into()
    } else {
        input
    }
}

/// Attribute to generate a [`rocket::request::FromRequest<'r>`] implementation for the struct applied to.
///
/// This attribute can only be applied to structs.
///
/// # Grammar
///
/// The grammar for the `#[rocket_variant_config]` attributes is defined as:
///
/// ```text
/// rocket_variant_config := Name* | File=Path
///
/// Name := configuration file name without extension, e.g. `rocket_variant_config("test")`
/// Path := path to the configuration file, e.g. `rocket_variant_config(file = "test.toml")`
/// ```
///
/// [`rocket::request::FromRequest<'r>`]: https://docs.rs/rocket/latest/rocket/request/trait.FromRequest.html
#[proc_macro_attribute]
pub fn rocket_variant_config(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    if cfg!(feature = "rocket") {
        attribute::rocket::variant_config(args.into(), input.into()).into()
    } else {
        input
    }
}

pub(crate) fn extract_file_path(
    args: proc_macro2::TokenStream,
) -> (Option<String>, Option<String>) {
    let att_meta = &syn::parse2::<devise::MetaItem>(args.into()).unwrap();
    match att_meta {
        devise::MetaItem::List {
            path: _,
            delimiter: _,
            items,
        } => match items.first().unwrap() {
            devise::MetaItem::Tokens(ts) => {
                let lit_str: syn::LitStr = syn::parse2(ts.clone()).unwrap();
                (Some(lit_str.value()), None)
            }
            devise::MetaItem::KeyValue {
                path,
                eq: _,
                tokens,
            } if path.get_ident().unwrap().to_string().to_lowercase() == "file" => {
                let lit_str: syn::LitStr = syn::parse2(tokens.clone()).unwrap();
                (None, Some(lit_str.value()))
            }
            _ => (None, None),
        },
        _ => (None, None),
    }
}
