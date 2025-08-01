use actix_web::Responder;
use vconfig_actix_web::serde::Deserialize;
use vconfig_actix_web::vconfig;

#[derive(Debug, Deserialize)]
#[serde(crate = "vconfig_actix_web::serde")]
#[vconfig("index")] // with configs in config store.
//#[vconfig("./src/configs/index.toml")] // relative path to wroking directory.
//#[vconfig(file = "./src/configs/index.toml")] // relative path to wroking directory.
pub(crate) struct IndexConfig {
    welcome_msg: String,
}

pub(crate) async fn index(index_config: IndexConfig) -> impl Responder {
    index_config.welcome_msg
}
