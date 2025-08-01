use vconfig_axum::serde::Deserialize;
use vconfig_axum::vconfig;

#[derive(Debug, Deserialize)]
#[serde(crate = "vconfig_axum::serde")]
#[vconfig("index")]
pub(crate) struct IndexConfig {
    welcome_msg: String,
}

pub(crate) async fn index(index_config: IndexConfig) -> String {
    index_config.welcome_msg
}
