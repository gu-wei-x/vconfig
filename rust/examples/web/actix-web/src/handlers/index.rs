use actix_web::Responder;
use variants_actix_web::de::variants_config;
use variants_actix_web::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "variants_actix_web::serde")]
#[variants_config("index")]
pub(crate) struct IndexConfig {
    welcome_msg: String,
}

pub(crate) async fn index(index_config: IndexConfig) -> impl Responder {
    index_config.welcome_msg
}
