#![allow(dead_code)]
use crate::parser::types::{Result, string, table::Table};
use winnow::stream::Stream;

use crate::{
    parser::{Token, types::value::Value},
    tokenizer::{
        stream::{self, TokenStream},
        token::TokenKind,
    },
};

#[derive(Clone, Debug)]
pub struct Array {
    // todo: all items should be some type for an entry.
    items: Vec<Value>,
}

impl Default for Array {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}

impl Array {
    pub(crate) fn add_item(&mut self, value: Value) -> &mut Self {
        self.items.push(value);
        self
    }
}

impl Array {
    pub(crate) fn from<'a>(
        source: &'a str,
        token_stream: &mut TokenStream,
        _token: &Token,
    ) -> Result<Value> {
        // <
        // any simple value must be string-style.
        //simple = <"a1","a2"> #simple array
        //#simple2 = <<"a1", "a2">, <"a3", "a4>>
        //#simple2 = <{a1="test", a2="test"}, {a1="test", a2="test"}>
        token_stream.next_token(); // comsume <.
        _ = stream::skip_whitespace_and_newline(token_stream);
        let mut array = Array::default();
        while let Some(current_token) = token_stream.peek_token() {
            match current_token.kind() {
                TokenKind::LESSTHAN => {
                    // array again
                    let array_value = Some(Array::from(source, token_stream, current_token)?);
                    if let Some(data) = array_value {
                        array.add_item(data);
                        stream::skip_next_token_if(token_stream, |k| {
                            vec![TokenKind::COMMA, TokenKind::WHITESPACE, TokenKind::NEWLINE]
                                .contains(&k)
                        });
                    } else {
                        return Result::from(current_token);
                    }
                }
                TokenKind::LCURLYBRACKET => {
                    // table
                    let table_value =
                        Some(Table::from(source, token_stream, current_token, false)?);
                    if let Some(data) = table_value {
                        array.add_item(data);
                        stream::skip_next_token_if(token_stream, |k| {
                            vec![TokenKind::COMMA, TokenKind::WHITESPACE, TokenKind::NEWLINE]
                                .contains(&k)
                        });
                    } else {
                        return Result::from(current_token);
                    }
                }
                TokenKind::GREATTHAN => {
                    // consume and break.
                    token_stream.next_token();
                    break;
                }
                TokenKind::COMMENT | TokenKind::NEWLINE | TokenKind::WHITESPACE => {
                    token_stream.next_token();
                    continue;
                }
                TokenKind::MLDOUBLEQUOTEDSTRING
                | TokenKind::DOUBLEQUOTEDSTRING
                | TokenKind::MLSINGLEQUOTEDSTRING
                | TokenKind::SINGLEQUOTEDSTRING => {
                    let raw_value = string::from(source, current_token);
                    match raw_value {
                        Ok(str) => {
                            token_stream.next_token();
                            array.add_item(Value::String(str.to_owned()));
                            // skip whitespace, nl, comma
                            stream::skip_next_token_if(token_stream, |k| {
                                vec![TokenKind::COMMA, TokenKind::WHITESPACE, TokenKind::NEWLINE]
                                    .contains(&k)
                            });
                        }
                        _ => return Result::from(current_token),
                    }
                }
                _ => {
                    // value.
                    return Result::from(current_token);
                }
            }
        }

        Ok(Value::Array(array))
    }
}
