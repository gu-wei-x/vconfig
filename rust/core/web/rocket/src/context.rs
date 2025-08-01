use rocket::Request;
use std::path::{Path, PathBuf};
use vconfig::fs::ConfigStore;
use vconfig::traits::Variants;

use crate::{VariantsProcessor, builder::VariantsBuilder};

pub struct VConfigContext {
    configs: ConfigStore,
    builder: VariantsBuilder,
}

impl VConfigContext {
    pub fn new(base_dir: &Path) -> Option<VConfigContext> {
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

    pub fn get_file(&self, name: &str) -> Option<PathBuf> {
        let path = std::path::PathBuf::from(name);
        if path.exists() {
            return Some(path);
        }

        let path = self.configs.get_path(name);
        match path {
            Some(path) => {
                if path.exists() {
                    Some(path.to_owned())
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn build_variants<'r>(&self, request: &'r Request<'_>, variants: &mut dyn Variants) {
        self.builder.build(request, variants);
    }

    pub fn with_processor<P: VariantsProcessor>(&mut self, processor: P) -> &mut Self {
        self.builder.with_processor(processor);
        self
    }
}
