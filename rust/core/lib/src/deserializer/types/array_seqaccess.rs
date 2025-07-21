use crate::deserializer::types::array::ArrayDeserializer;
use crate::deserializer::types::value::ValueDeserializer;
use crate::types::error;
use crate::types::traits::Variants;
use crate::types::value::Value;

pub(crate) struct ArraySeqAccess<'a, 'b, V>
where
    V: Variants,
{
    iterator: std::slice::Iter<'a, Value>,
    variants: &'b V,
}

impl<'a, 'b, V> ArraySeqAccess<'a, 'b, V>
where
    V: Variants,
{
    pub(crate) fn new(deserilizer: &'a ArrayDeserializer<'b, V>) -> Self {
        Self {
            iterator: deserilizer.data.into_vec().iter(),
            variants: deserilizer.variants,
        }
    }
}

impl<'a, 'de, V> serde::de::SeqAccess<'de> for ArraySeqAccess<'a, 'de, V>
where
    V: Variants,
{
    type Error = error::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        match self.iterator.next() {
            Some(value) => {
                let value_deserializer = ValueDeserializer::new(value.clone(), self.variants);
                seed.deserialize(value_deserializer).map(Some)
            }
            None => Ok(None),
        }
    }
}
