#![allow(dead_code)]
use winnow::stream::Stream;

use crate::parser::Token;
use crate::parser::types::Result;
use crate::parser::types::array::Array;
use crate::parser::types::string;
use crate::parser::types::table::Table;
use crate::tokenizer::stream::TokenStream;
use crate::tokenizer::token::TokenKind;

#[derive(Clone /*, Debug*/)]
pub enum Value {
    // simple value must be string style.
    String(String),
    Array(Array),
    Table(Table),
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(str) => write!(f, "Value({:#?})", str)?,
            Value::Array(array) => write!(f, "Value(<{:#?}>)", array)?,
            Value::Table(table) => write!(f, "Value({:#?})", table)?,
        }
        Ok(())
    }
}

impl Value {
    pub(crate) fn string_value(&mut self) -> Option<&str> {
        // todo: impl
        None
    }

    pub(crate) fn array_mut(&mut self) -> Option<&mut Array> {
        if let Value::Array(array) = self {
            Some(array)
        } else {
            None
        }
    }

    pub(crate) fn table_mut(&mut self) -> Option<&mut Table> {
        if let Value::Table(table) = self {
            Some(table)
        } else {
            None
        }
    }
}

impl Value {
    pub(crate) fn from<'a>(
        source: &'a str,
        token_stream: &mut TokenStream,
        previous_token: &Token,
    ) -> Result<Value> {
        if let Some(token) = token_stream.peek_token() {
            match token.kind() {
                TokenKind::DOUBLEQUOTEDSTRING
                | TokenKind::SINGLEQUOTEDSTRING
                | TokenKind::MLDOUBLEQUOTEDSTRING
                | TokenKind::MLSINGLEQUOTEDSTRING => {
                    // must be string.
                    let raw_value = string::from(source, token);
                    token_stream.next_token(); // consume the token.
                    match raw_value {
                        Ok(str) => Ok(Value::String(str.to_owned())),
                        _ => Result::from(token),
                    }
                }
                TokenKind::LESSTHAN => {
                    // inlined array.
                    Array::from(source, token_stream, token)
                }
                TokenKind::LCURLYBRACKET => {
                    let table_result = Table::from(source, token_stream, token, false);
                    match table_result {
                        Ok(table) => Ok(table),
                        _ => Result::from(token),
                    }
                }
                TokenKind::EOF => Result::from(token), // no value.
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
