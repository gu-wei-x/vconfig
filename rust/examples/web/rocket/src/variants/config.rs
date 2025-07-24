extern crate variants as variantslib;
use crate::variants::builder::VariantsBuilder;
use normpath::PathExt;
use rocket::Request;
use std::path::{Path, PathBuf};
use variantslib::default::DefaultVariants;

const EXT: &'static str = "toml";
pub(crate) struct VaraintsConfig {
    configs: variantslib::fs::ConfigStore,
    builder: VariantsBuilder,
}

impl VaraintsConfig {
    pub(crate) fn new(config_root: &Path) -> Option<VaraintsConfig> {
        let config_root = match config_root.normalize() {
            Ok(config_root) => config_root.into_path_buf(),
            _ => {
                return None;
            }
        };

        let mut config_store = variantslib::fs::ConfigStore::new(&config_root.to_string_lossy());
        config_store.with_ext(EXT);
        config_store.init();

        Some(Self {
            configs: config_store,
            builder: VariantsBuilder::new(),
        })
    }

    pub(crate) fn get_file(&self, name: &str) -> Option<&PathBuf> {
        self.configs.get_path(name)
    }

    pub(crate) fn build_varaints<'r>(
        &self,
        request: &'r Request<'_>,
        variants: &mut DefaultVariants,
    ) {
        self.builder.build(request, variants);
    }
}
