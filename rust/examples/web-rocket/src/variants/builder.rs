extern crate variants as variantslib;
use crate::variants::browser::BrowserVaraints;
use rocket::Request;

pub trait VariantsProcessor<'r, V>
where
    V: variantslib::traits::Variants,
{
    fn process(&self, request: &'r Request<'_>, variants: &mut V);
}

pub(crate) struct VariantsBuilder<'r, V>
where
    V: variantslib::traits::Variants,
{
    processors: Vec<Box<dyn VariantsProcessor<'r, V>>>,
}

impl<'r, V> Default for VariantsBuilder<'r, V>
where
    V: variantslib::traits::Variants,
{
    fn default() -> Self {
        Self {
            processors: Vec::new(),
        }
    }
}

impl<'r, V> VariantsBuilder<'r, V>
where
    V: variantslib::traits::Variants,
{
    pub(crate) fn config(&mut self) -> &mut Self {
        self.with_processor(Box::new(BrowserVaraints::default()));
        self
    }

    pub(crate) fn build(&self, request: &'r Request<'_>, variants: &mut V) {
        for iter in self.processors.iter() {
            iter.process(request, variants);
        }
    }

    fn with_processor(&mut self, processor: Box<dyn VariantsProcessor<'r, V>>) -> &mut Self {
        self.processors.push(processor);
        self
    }
}
