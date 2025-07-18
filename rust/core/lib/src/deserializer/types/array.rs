use crate::{
    deserializer::types::array_seqaccess::ArraySeqAccess,
    types::{array::Array, error, traits::Variants},
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
    type Error = error::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let map_access = ArraySeqAccess::new(&self);
        visitor.visit_seq(map_access)
    }
    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string
        seq bytes byte_buf map unit ignored_any option enum unit_struct
        tuple_struct tuple identifier
    }
}
