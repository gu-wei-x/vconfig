use rocket::get;
use vconfig_rocket::serde::Deserialize;
use vconfig_rocket::vconfig;

#[derive(Debug, Deserialize)]
#[serde(crate = "vconfig_rocket::serde")]
#[vconfig("index")]
pub(crate) struct IndexConfig {
    welcome_msg: String,
}

#[get("/")]
pub(crate) async fn index(index_config: IndexConfig) -> String {
    index_config.welcome_msg
}
