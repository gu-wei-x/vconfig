use std::sync::Arc;
use variants_de::default::DefaultVariants;

pub trait VariantsProcessor: Send + Sync + 'static {
    fn process<'r>(&self, request: &actix_web::HttpRequest, variants: &mut DefaultVariants);
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

    pub(crate) fn build(&self, request: &actix_web::HttpRequest, variants: &mut DefaultVariants) {
        for iter in self.processors.iter() {
            iter.process(request, variants);
        }
    }

    pub(crate) fn with_processor<P: VariantsProcessor>(&mut self, processor: P) -> &mut Self {
        self.processors.push(Arc::new(processor));
        self
    }
}
