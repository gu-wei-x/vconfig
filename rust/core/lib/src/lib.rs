mod de;
pub mod parser;
pub use crate::parser::error::Error;
pub use crate::parser::tokenizer;
use std::collections::HashMap;

pub fn from_str<'a, T>(source: &'a str, _variants: &'a HashMap<String, String>) -> Result<T, String>
where
    T: serde::de::Deserialize<'a>,
{
    let result = parser::parse_str(source);
    match result {
        Ok(table) => {
            // todo: Implement deserialization from table to type T
            // For now, we will return an error indicating that this is not implemented yet.
            // T::deserialize(Deserializer::parse(s)?)
            Err(format!(
                "Deserialization from table to type T is not implemented yet: {:?}",
                table
            ))
        }
        Err(e) => Err(format!("Parsing error: {:?}", e)),
    }
}
