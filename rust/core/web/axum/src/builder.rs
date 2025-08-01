use axum::http::request::Parts;
use std::sync::Arc;
use vconfig::traits::Variants;

/// A trait for processing variants in a web context.
///
/// Types implementing this trait can perform custom processing on a set of variants,
/// typically used for modifying or inspecting request parts and associated variant data.
///
/// # Safety
/// Implementors must ensure thread safety as this trait requires `Send` and `Sync`.
///
/// # Arguments
/// * `parts` - Reference to the request parts to be processed.
/// * `variants` - Mutable reference to the variants to be processed or modified.
pub trait VariantsProcessor: Send + Sync + 'static {
    fn process<'r>(&self, parts: &'r Parts, variants: &mut dyn Variants);
}

#[derive(Clone)]
pub(crate) struct VariantsBuilder {
    processors: Vec<Arc<dyn VariantsProcessor>>,
}

impl VariantsBuilder {
    pub(crate) fn new() -> Self {
        Self {
            processors: Vec::new(),
        }
    }

    pub(crate) fn build<'r>(&self, parts: &'r Parts, variants: &mut dyn Variants) {
        for iter in self.processors.iter() {
            iter.process(parts, variants);
        }
    }

    pub(crate) fn with_processor<P: VariantsProcessor>(&mut self, processor: P) -> &mut Self {
        self.processors.push(Arc::new(processor));
        self
    }
}
