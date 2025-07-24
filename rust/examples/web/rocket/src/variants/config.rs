extern crate variants as variantslib;
use normpath::PathExt;
use rocket::Request;
use std::path::{Path, PathBuf};
use variantslib::default::DefaultVariants;

pub(crate) struct VaraintsContext {
    configs: variants_rocket::fs::ConfigStore,
    builder: variants_rocket::VariantsBuilder,
}

impl VaraintsContext {
    pub(crate) fn new(config_root: &Path) -> Option<VaraintsContext> {
        let config_root = match config_root.normalize() {
            Ok(config_root) => config_root.into_path_buf(),
            _ => {
                return None;
            }
        };

        // config.
        let mut config_store = variantslib::fs::ConfigStore::new(&config_root.to_string_lossy());
        config_store.with_ext("toml");
        config_store.init();

        // variants.
        let mut variants_builder = variants_rocket::VariantsBuilder::new();
        variants_builder.with_processor(crate::variants::browser::BrowserVaraints::default());

        Some(Self {
            configs: config_store,
            builder: variants_builder,
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
