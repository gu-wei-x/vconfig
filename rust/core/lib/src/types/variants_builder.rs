#![allow(dead_code)]
use crate::traits::Variants;

pub trait VariantsProcessor<D, V>
where
    V: Variants,
{
    fn process(&self, data: &D, variants: &mut V);
}

pub struct VariantsBuilder<D, V>
where
    V: Variants,
{
    processers: Vec<Box<dyn VariantsProcessor<D, V>>>,
}

impl<D, V> Default for VariantsBuilder<D, V>
where
    V: Variants + Default,
{
    fn default() -> Self {
        Self {
            processers: Vec::new(),
        }
    }
}

impl<D, V> VariantsBuilder<D, V>
where
    V: Variants + Default,
{
    pub fn with_processor(&mut self, processor: Box<dyn VariantsProcessor<D, V>>) -> &mut Self {
        self.processers.push(processor);
        self
    }

    // get variants from context data.
    pub fn process_variants(&self, data: &D, varaints: &mut V) {
        for iter in self.processers.iter() {
            iter.process(data, varaints);
        }
    }
}
