pub(crate) mod array;
pub(crate) mod string;
pub(crate) mod table;
pub(crate) mod value;

#[cfg(test)]
pub(crate) mod tests;

use crate::parser::{Token, tokenizer};
use crate::types::result::Result;
use crate::types::table::Table;
use crate::types::value::Value;
use winnow::stream::{Stream as _, TokenSlice};

pub(crate) fn parse_str<'a>(source: &'a str) -> Result<Table> {
    let tokenizer = tokenizer::Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream: TokenSlice<'_, Token> = TokenSlice::new(&tokens);
    if let Some(token) = token_stream.next_token() {
        let value_result = Table::from(source, &mut token_stream, token, false);
        match value_result {
            Ok(value) => {
                if let Value::Table(table) = value {
                    Ok(table)
                } else {
                    Result::from(token)
                }
            }
            _ => Result::from(token),
        }
    } else {
        Ok(Table::default())
    }
}
