use std::path::{Path, PathBuf};

use crate::{VariantsProcessor, builder::VariantsBuilder};
use vconfig::fs::ConfigStore;
use vconfig::traits::Variants;

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

    pub fn get_file(&self, name: &str) -> Option<&PathBuf> {
        self.configs.get_path(name)
    }

    pub fn build_variants<'r>(
        &self,
        request: &actix_web::HttpRequest,
        variants: &mut dyn Variants,
    ) {
        self.builder.build(request, variants);
    }

    pub fn with_processor<P: VariantsProcessor>(&mut self, processor: P) -> &mut Self {
        self.builder.with_processor(processor);
        self
    }
}
