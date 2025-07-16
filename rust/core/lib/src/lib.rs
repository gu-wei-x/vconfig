mod de;
mod parser;

use crate::parser::{
    Token,
    tokenizer::{self, token::Kind},
    types::table::Table,
};
use std::collections::HashMap;

pub fn from_str<'a, T>(source: &'a str, _variants: &'a HashMap<String, String>) -> Result<T, Token>
where
    T: serde::de::Deserialize<'a>,
{
    let result: crate::parser::types::Result<Table> = parser::parse_str(source);
    match result {
        Ok(_table) => {
            // todo: Implement deserialization from table to type T
            // For now, we will return an error indicating that this is not implemented yet.
            // T::deserialize(Deserializer::parse(s)?)
            //T::deserialize(table)?
            let token = Token::new(Kind::UNKNOWN, 0, 0);
            crate::parser::types::Result::from(token)
        }
        Err(token) => crate::parser::types::Result::from(token),
    }
}
