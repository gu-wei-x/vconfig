extern crate variants as variantslib;
use crate::variants::browser::BrowserVaraints;
use rocket::Request;
use std::sync::Arc;
use variantslib::default::DefaultVariants;

pub trait VariantsProcessor: Send + Sync + 'static {
    fn process<'r>(&self, request: &'r Request<'_>, variants: &mut DefaultVariants);
}

#[derive(Clone)]
pub(crate) struct VariantsBuilder {
    processors: Vec<Arc<dyn VariantsProcessor>>,
}

impl VariantsBuilder {
    pub(crate) fn new() -> Self {
        let result = Self {
            processors: Vec::new(),
        };
        result.with_processor(BrowserVaraints::default())
    }

    pub(crate) fn build<'r>(&self, request: &'r Request<'_>, variants: &mut DefaultVariants) {
        for iter in self.processors.iter() {
            iter.process(request, variants);
        }
    }

    fn with_processor<P: VariantsProcessor>(mut self, processor: P) -> Self {
        self.processors.push(Arc::new(processor));
        self
    }
}
