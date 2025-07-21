#![allow(dead_code)]
use crate::traits::Variants;

pub trait VariantsProcesser<D, V>
where
    V: Variants,
{
    fn process(&self, data: &D, variants: &V);
}

pub struct VariantsBuilder<D, V>
where
    V: Variants,
{
    processers: Vec<Box<dyn VariantsProcesser<D, V>>>,
    variants: V,
}

impl<D, V> Default for VariantsBuilder<D, V>
where
    V: Variants + Default,
{
    fn default() -> Self {
        Self {
            variants: V::default(),
            processers: Vec::new(),
        }
    }
}

impl<D, V> VariantsBuilder<D, V>
where
    V: Variants + Default,
{
    fn add_processor(&mut self, processor: &dyn VariantsProcesser<D, V>) {
        self.processers.push(processor);
    }

    // get variants from context data.
    fn get_variants(&self, data: &D, varaints: &mut V) {
        for iter in self.processers.iter() {
            iter.process(data, varaints);
        }
    }
}
