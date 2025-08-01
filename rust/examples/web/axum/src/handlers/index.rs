use vconfig_axum::de::vconfig;
use vconfig_axum::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "vconfig_axum::serde")]
#[vconfig("index")]
pub(crate) struct IndexConfig {
    welcome_msg: String,
}

pub(crate) async fn index(index_config: IndexConfig) -> String {
    index_config.welcome_msg
}
