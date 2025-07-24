#![allow(unused_imports)]
extern crate variants as variantslib;
use variants_rocket::config;
use variantslib::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "variantslib::serde")]
#[config("index")]
pub(crate) struct IndexConfig {
    pub(crate) welcome_msg: String,
}
