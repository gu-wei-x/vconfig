use rocket::Request;
use std::sync::Arc;
use vconfig::traits::Variants;

/// A trait for processing variants in the context of a web request.
///
/// Types implementing this trait can modify or inspect the provided `variants`
/// based on the incoming `request`. Implementors must be thread-safe and have
/// a static lifetime.
///
/// # Arguments
///
/// * `request` - A reference to the current HTTP request.
/// * `variants` - A mutable reference to a type implementing the `Variants` trait,
///   allowing the processor to modify or inspect the available variants.
pub trait VariantsProcessor: Send + Sync + 'static {
    fn process<'r>(&self, request: &'r Request<'_>, variants: &mut dyn Variants);
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

    pub(crate) fn build<'r>(&self, request: &'r Request<'_>, variants: &mut dyn Variants) {
        for iter in self.processors.iter() {
            iter.process(request, variants);
        }
    }

    pub(crate) fn with_processor<P: VariantsProcessor>(&mut self, processor: P) -> &mut Self {
        self.processors.push(Arc::new(processor));
        self
    }
}
