use crate::parser::types::string;
use crate::types::array::Array;
use crate::types::result::Result;
use crate::types::table::Table;
use crate::types::value::Value;
use winnow::stream::Stream;

use crate::{
    parser::Token,
    tokenizer::{
        stream::{self, TokenStream},
        token::Kind,
    },
};

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
                Kind::LESSTHAN => {
                    // array again
                    let array_value = Some(Array::from(source, token_stream, current_token)?);
                    if let Some(data) = array_value {
                        array.push(data)?;
                        stream::skip_next_token_if(token_stream, |k| {
                            vec![Kind::COMMA, Kind::WHITESPACE, Kind::NEWLINE].contains(&k)
                        });
                    } else {
                        return Result::from(current_token);
                    }
                }
                Kind::LCURLYBRACKET => {
                    // table
                    let table_value =
                        Some(Table::from(source, token_stream, current_token, false)?);
                    if let Some(data) = table_value {
                        array.push(data)?;
                        stream::skip_next_token_if(token_stream, |k| {
                            vec![Kind::COMMA, Kind::WHITESPACE, Kind::NEWLINE].contains(&k)
                        });
                    } else {
                        return Result::from(current_token);
                    }
                }
                Kind::GREATTHAN => {
                    // consume and break.
                    token_stream.next_token();
                    break;
                }
                Kind::COMMENT | Kind::NEWLINE | Kind::WHITESPACE => {
                    token_stream.next_token();
                    continue;
                }
                Kind::STRING => {
                    let raw_value = string::from(source, current_token);
                    match raw_value {
                        Ok(str) => {
                            token_stream.next_token();
                            array.push(Value::String(str.to_owned()))?;
                            // skip whitespace, nl, comma
                            stream::skip_next_token_if(token_stream, |k| {
                                vec![Kind::COMMA, Kind::WHITESPACE, Kind::NEWLINE].contains(&k)
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
