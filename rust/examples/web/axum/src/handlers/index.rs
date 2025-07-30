use variants_axum::de::variants_config;
use variants_axum::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "variants_axum::serde")]
#[variants_config("index")]
pub(crate) struct IndexConfig {
    welcome_msg: String,
}

pub(crate) async fn index(index_config: IndexConfig) -> String {
    index_config.welcome_msg
}
