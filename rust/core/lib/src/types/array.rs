use crate::types::error;
use crate::types::result;
use crate::types::value::Value;

#[derive(Clone, Debug, PartialEq)]
pub struct Array {
    items: Vec<Value>,
}

impl Default for Array {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}

impl Array {
    // validate type and push value.
    pub(crate) fn push(&mut self, value: Value) -> result::Result<()> {
        let last = self.items.last();
        match last {
            Some(last) => match std::mem::discriminant(last) == std::mem::discriminant(&value) {
                true => {
                    self.items.push(value);
                    Ok(())
                }
                false => error::Error::from_str("Array items must have some type.").into(),
            },
            None => {
                self.items.push(value);
                Ok(())
            }
        }
    }

    pub(crate) fn into_vec(&self) -> &Vec<Value> {
        &self.items
    }
}
