use crate::deserializer::types::array;
use crate::deserializer::types::table;
use crate::parser::Token;
use crate::types::traits::Variants;
use crate::types::value::Value;

pub(crate) type ValueDeserializer<'a, V> = crate::deserializer::types::Deserializer<'a, V, Value>;

impl<'a, V> ValueDeserializer<'a, V>
where
    V: Variants,
{
    pub(crate) fn new(data: Value, variants: &'a V) -> Self {
        Self {
            data: data,
            variants: variants,
        }
    }
}

impl<'de, T> serde::Deserializer<'de> for ValueDeserializer<'de, T>
where
    T: Variants,
{
    type Error = Token;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.data {
            Value::String(str) => visitor.visit_string(str),
            Value::Array(array) => {
                let array_deserializer = array::ArrayDeserializer::new(array, self.variants);
                array_deserializer.deserialize_any(visitor)
            }
            Value::Table(table) => {
                let table_deserializer = table::TableDeserializer::new(table, self.variants);
                table_deserializer.deserialize_any(visitor)
            }
        }
    }

    // todo: define a new macro to do forward_to_deserialize_any but all primitives types are from str
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.data {
            Value::String(str) => visitor.visit_u64(str.parse::<u64>().unwrap()),
            _ => self.deserialize_any(visitor),
        }
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
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }

    serde::forward_to_deserialize_any! {
        bool u8 u16 u32 i8 i16 i32 i64 f32 f64 char str string
        seq bytes byte_buf map unit ignored_any option enum unit_struct
        tuple_struct tuple identifier
    }
}
