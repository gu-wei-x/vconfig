use normpath::PathExt;
use rocket::Request;
use std::path::{Path, PathBuf};
use vconfig::default::DefaultVariants;

use crate::{VariantsProcessor, builder::VariantsBuilder};
use vconfig::fs::ConfigStore;

pub struct VariantsContext {
    configs: ConfigStore,
    builder: VariantsBuilder,
}

impl VariantsContext {
    pub fn new(base_dir: &Path) -> Option<VariantsContext> {
        let base_dir = match base_dir.normalize() {
            Ok(base_dir) => base_dir.into_path_buf(),
            _ => {
                return None;
            }
        };

        // config.
        let mut config_store = ConfigStore::new(&base_dir.to_string_lossy());
        config_store.with_ext("toml");
        config_store.init();

        // variants.
        let variants_builder = VariantsBuilder::new();
        Some(Self {
            configs: config_store,
            builder: variants_builder,
        })
    }

    pub fn get_file(&self, name: &str) -> Option<&PathBuf> {
        self.configs.get_path(name)
    }

    pub fn build_variants<'r>(&self, request: &'r Request<'_>, variants: &mut DefaultVariants) {
        self.builder.build(request, variants);
    }

    pub fn with_processor<P: VariantsProcessor>(&mut self, processor: P) -> &mut Self {
        self.builder.with_processor(processor);
        self
    }
}
