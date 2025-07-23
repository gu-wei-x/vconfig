#![allow(unused_imports)]
extern crate variants as variantslib;
use variantslib::serde::Deserialize;
use variants_rocket::config;

#[derive(Debug, Deserialize)]
#[serde(crate = "variantslib::serde")]
#[config("index")]
pub(crate) struct IndexConfig {
    pub(crate) welcome_msg: String,
}
