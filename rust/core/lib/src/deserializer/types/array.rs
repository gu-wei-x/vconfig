#![allow(dead_code)]

use crate::{
    parser::Token,
    types::{array::Array, traits::Variants},
};

pub(crate) type ArrayDeserializer<'a, V> = crate::deserializer::types::Deserializer<'a, V, Array>;

impl<'a, V> ArrayDeserializer<'a, V>
where
    V: Variants,
{
    pub(crate) fn new(data: Array, variants: &'a V) -> Self {
        Self {
            data: data,
            variants: variants,
        }
    }
}

impl<'de, T> serde::Deserializer<'de> for ArrayDeserializer<'de, T>
where
    T: Variants,
{
    type Error = Token;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }
    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        todo!()
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string
        seq bytes byte_buf map unit ignored_any option enum unit_struct
        tuple_struct tuple identifier
    }
}
