#![allow(dead_code)]
use crate::types::value::Value;

#[derive(Clone, Debug)]
pub struct Array {
    // todo: all items should be some type for an entry.
    items: Vec<Value>,
}

impl Default for Array {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}

impl Array {
    pub(crate) fn add_item(&mut self, value: Value) -> &mut Self {
        self.items.push(value);
        self
    }
}
