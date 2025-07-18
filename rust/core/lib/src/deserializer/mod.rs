#![allow(dead_code)]
mod types;
use crate::deserializer::types::value;
use crate::parser::Token;
use crate::types::table::Table;
use crate::types::traits::Variants;
use crate::types::value::Value;

pub struct Deserializer<'a, T>
where
    T: Variants,
{
    root: Table,
    variants: &'a T,
}

impl<'a, T> Deserializer<'a, T>
where
    T: Variants,
{
    pub(crate) fn new(data: Table, variants: &'a T) -> Self {
        Self {
            root: data,
            variants: variants,
        }
    }
}

// https://docs.rs/serde/latest/serde/de/trait.Deserializer.html
impl<'de, T> serde::Deserializer<'de> for Deserializer<'de, T>
where
    T: Variants,
{
    type Error = Token;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let value_deserializer =
            value::ValueDeserializer::new(Value::Table(self.root), self.variants);
        value_deserializer.deserialize_any(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let value_deserializer =
            value::ValueDeserializer::new(Value::Table(self.root), self.variants);
        value_deserializer.deserialize_newtype_struct(name, visitor)
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let value_deserializer =
            value::ValueDeserializer::new(Value::Table(self.root), self.variants);
        value_deserializer
            .deserialize_struct(name, fields, visitor)
            .map_err(|e: Self::Error| e)
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string
        seq bytes byte_buf map unit ignored_any option enum unit_struct
        tuple_struct tuple identifier
    }
}
