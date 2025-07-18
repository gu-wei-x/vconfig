use crate::deserializer::types::table::TableDeserializer;
use crate::deserializer::types::value::ValueDeserializer;
use crate::types::entry::VariantEntry;
use crate::types::error;
use crate::types::traits::Variants;
use serde::de::IntoDeserializer;

pub(crate) struct MapAccess<'a, 'b, V> {
    iterator: indexmap::map::Iter<'a, String, VariantEntry>,
    value: Option<VariantEntry>,
    variants: &'b V,
}

impl<'a, 'b, V> MapAccess<'a, 'b, V>
where
    V: Variants,
{
    pub(crate) fn new(input: &'a TableDeserializer<'b, V>) -> Self {
        Self {
            iterator: input.data.into_map().iter(),
            value: None,
            variants: input.variants,
        }
    }
}

impl<'a, 'de, V> serde::de::MapAccess<'de> for MapAccess<'a, 'de, V>
where
    V: Variants,
{
    type Error = error::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        println!("***********************************************");
        match self.iterator.next() {
            Some((key, entry)) => {
                self.value = Some(entry.clone());

                println!("++++++++++++++++++{:#?}+++++++++++++++++++", key);
                seed.deserialize(key.clone().into_deserializer()).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<S>(&mut self, seed: S) -> Result<S::Value, Self::Error>
    where
        S: serde::de::DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(entry) => {
                // get envaluated one.
                match entry.find_item(self.variants).take() {
                    Some(value) => {
                        println!("++++++++++++++++++{:#?}+++++++++++++++++++", value);
                        let value_deserializer =
                            ValueDeserializer::new(value.clone(), self.variants);
                        seed.deserialize(value_deserializer)
                    }
                    None => {
                        panic!(
                            "no more values in next_value_seed, internal error in ValueDeserializer"
                        )
                    }
                }
            }
            None => {
                panic!("no more values in next_value_seed, internal error in ValueDeserializer")
            }
        }
    }
}
