#![allow(dead_code)]
use winnow::stream::Stream;

use crate::parser::Token;
use crate::parser::types::string;
use crate::tokenizer::stream::TokenStream;
use crate::tokenizer::token::Kind;
use crate::types::array::Array;
use crate::types::result::Result;
use crate::types::table::Table;
use crate::types::value::Value;

impl Value {
    pub(crate) fn from<'a>(
        source: &'a str,
        token_stream: &mut TokenStream,
        previous_token: &Token,
    ) -> Result<Value> {
        if let Some(token) = token_stream.peek_token() {
            match token.kind() {
                Kind::STRING => {
                    // must be string.
                    let raw_value = string::from(source, token);
                    token_stream.next_token(); // consume the token.
                    match raw_value {
                        Ok(str) => Ok(Value::String(str.to_owned())),
                        _ => Result::from(token),
                    }
                }
                Kind::LESSTHAN => {
                    // inlined array.
                    Array::from(source, token_stream, token)
                }
                Kind::LCURLYBRACKET => {
                    let table_result = Table::from(source, token_stream, token, false);
                    match table_result {
                        Ok(table) => Ok(table),
                        _ => Result::from(token),
                    }
                }
                Kind::EOF => Result::from(token), // no value.
                _ => {
                    // invalid token.
                    Result::from(token)
                }
            }
        } else {
            // no value.
            Result::from(previous_token)
        }
    }
}
