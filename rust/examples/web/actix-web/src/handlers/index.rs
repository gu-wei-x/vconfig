use actix_web::Responder;
use vconfig_actix_web::de::vconfig;
use vconfig_actix_web::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "vconfig_actix_web::serde")]
#[vconfig("index")]
pub(crate) struct IndexConfig {
    welcome_msg: String,
}

pub(crate) async fn index(index_config: IndexConfig) -> impl Responder {
    index_config.welcome_msg
}
