use crate::types::{error, table::Table, traits::Variants};

use crate::deserializer::types::table_mapaccess::MapAccess;

pub(crate) type TableDeserializer<'a, V> = crate::deserializer::types::Deserializer<'a, V, Table>;

impl<'a, V> TableDeserializer<'a, V>
where
    V: Variants,
{
    pub(crate) fn new(data: Table, variants: &'a V) -> Self {
        Self {
            data: data,
            variants: variants,
        }
    }
}

impl<'de, T> serde::Deserializer<'de> for TableDeserializer<'de, T>
where
    T: Variants,
{
    type Error = error::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        let map_access = MapAccess::new(&self);
        visitor.visit_map(map_access)
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
