mod deserializer;
mod parser;
mod types;

use types::table::Table;

use crate::{
    deserializer::Deserializer,
    parser::{
        Token,
        tokenizer::{self},
    },
};
use std::collections::HashMap;

/*pub fn from_str<'a, T>(source: &'a str) -> Result<T, Token>
where
    T: serde::de::Deserialize<'a>,
{
    let variants: HashMap<String, String> = std::collections::HashMap::new();
    from_str_with_variants(source, variants)
}*/

pub fn from_str_with_variants<'a: 'b, 'b, T>(
    source: &'a str,
    variants: &'b HashMap<String, String>,
) -> Result<T, Token>
where
    T: serde::de::Deserialize<'b>,
{
    let result: crate::parser::types::Result<Table> = parser::parse_str(source);
    match result {
        Ok(table) => {
            let deserializer = Deserializer::new(table, variants);
            let obj = T::deserialize(deserializer)?;
            Ok(obj)
        }
        Err(token) => crate::parser::types::Result::from(token),
    }
}
