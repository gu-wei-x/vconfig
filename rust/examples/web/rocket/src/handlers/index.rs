use rocket::get;
use vconfig_rocket::serde::Deserialize;
use vconfig_rocket::vconfig;

#[derive(Debug, Deserialize)]
#[serde(crate = "vconfig_rocket::serde")]
#[vconfig("index")] // with configs in config store.
//#[vconfig("./src/configs/index.toml")] // relative path to wroking directory.
//#[vconfig(file = "./src/configs/index.toml")] // relative path to wroking directory.
pub(crate) struct IndexConfig {
    welcome_msg: String,
}

#[get("/")]
pub(crate) async fn index(index_config: IndexConfig) -> String {
    index_config.welcome_msg
}
