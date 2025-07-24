mod context;

#[doc(inline)]
pub use variants_codegen::*;

use rocket::Request;
use std::sync::Arc;
use variants::default::DefaultVariants;

pub mod fs {
    pub use variants::fs::ConfigStore;
}

pub trait VariantsProcessor: Send + Sync + 'static {
    fn process<'r>(&self, request: &'r Request<'_>, variants: &mut DefaultVariants);
}

#[derive(Clone)]
pub struct VariantsBuilder {
    processors: Vec<Arc<dyn VariantsProcessor>>,
}

impl VariantsBuilder {
    pub fn new() -> Self {
        Self {
            processors: Vec::new(),
        }
    }

    pub fn build<'r>(&self, request: &'r Request<'_>, variants: &mut DefaultVariants) {
        for iter in self.processors.iter() {
            iter.process(request, variants);
        }
    }

    pub fn with_processor<P: VariantsProcessor>(&mut self, processor: P) -> &mut Self {
        self.processors.push(Arc::new(processor));
        self
    }
}
