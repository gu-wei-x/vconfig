use actix_web::Responder;
use vconfig_actix_web::de::variants_config;
use vconfig_actix_web::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "vconfig_actix_web::serde")]
#[variants_config("index")]
pub(crate) struct IndexConfig {
    welcome_msg: String,
}

pub(crate) async fn index(index_config: IndexConfig) -> impl Responder {
    index_config.welcome_msg
}
