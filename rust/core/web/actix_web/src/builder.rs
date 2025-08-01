use std::sync::Arc;
use vconfig::traits::Variants;

/// A trait for processing variant data in the context of an Actix Web request.
///
/// Types implementing this trait can modify or inspect the provided `Variants`
/// based on the incoming `HttpRequest`. This is typically used for content
/// negotiation, localization, or other request-dependent variant selection.
///
/// # Parameters
/// - `request`: The current Actix Web HTTP request.
/// - `variants`: A mutable reference to a type implementing the `Variants` trait,
///   which can be modified according to the request context.
///
/// # Thread Safety
/// Implementors must be `Send`, `Sync`, and `'static` to ensure safe usage
/// across threads and throughout the application's lifetime.
pub trait VariantsProcessor: Send + Sync + 'static {
    fn process<'r>(&self, request: &actix_web::HttpRequest, variants: &mut dyn Variants);
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

    pub(crate) fn build(&self, request: &actix_web::HttpRequest, variants: &mut dyn Variants) {
        for iter in self.processors.iter() {
            iter.process(request, variants);
        }
    }

    pub(crate) fn with_processor<P: VariantsProcessor>(&mut self, processor: P) -> &mut Self {
        self.processors.push(Arc::new(processor));
        self
    }
}
