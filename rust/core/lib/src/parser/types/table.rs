use crate::parser::types::{Result, string};
use crate::types::array::Array;
use crate::types::table::Table;
use crate::types::value::Value;
use crate::{
    parser::Token,
    parser::tokenizer::{
        stream::{self, TokenStream},
        token::Kind,
    },
};
use winnow::stream::Stream;

impl Table {
    pub(crate) fn from<'a>(
        source: &'a str,
        token_stream: &mut TokenStream,
        token: &Token,
        is_nested_table: bool,
    ) -> Result<Value> {
        if token.kind() == Kind::LCURLYBRACKET {
            token_stream.next_token(); // TokenKind::LCURLYBRACKET.
        }
        _ = stream::skip_meanlingless(token_stream);
        let mut table = Table::default();
        while let Some(current_token) = token_stream.peek_token() {
            match current_token.kind() {
                Kind::LSQUARBRACKET => {
                    if !is_nested_table {
                        Self::on_sub_container(source, token_stream, current_token, &mut table)?;
                    } else {
                        // let caller to handler next table.
                        break;
                    }
                }
                Kind::RCURLYBRACKET => {
                    // consume and break.
                    token_stream.next_token();
                    break;
                }
                Kind::EXP => {
                    // key value.
                    on_key_value_expression(source, token_stream, current_token, &mut table)?;
                }
                Kind::WHITESPACE | Kind::NEWLINE | Kind::COMMENT | Kind::COMMA => {
                    stream::skip_next_token_if(token_stream, |k| {
                        vec![Kind::COMMA, Kind::COMMENT, Kind::NEWLINE, Kind::WHITESPACE]
                            .contains(&k)
                    });
                }
                Kind::EOF => break,
                _ => {
                    // value.
                    return Result::from(current_token);
                }
            }
        }

        Ok(Value::Table(table))
    }

    fn on_sub_container<'a>(
        source: &'a str,
        token_stream: &mut TokenStream,
        start_token: &Token,
        container: &mut Table,
    ) -> Result<()> {
        // consume and ignore comment, whitespace, nl.
        token_stream.next_token();
        _ = stream::skip_meanlingless(token_stream);

        let mut table = container;
        let mut key_result: Result<&str> = Result::from(start_token);
        let mut variant_result: Result<&str> = Result::from(start_token);
        while let Some(token) = token_stream.next_token() {
            match token.kind() {
                Kind::DOT => {
                    // create parent entry.
                    if let Ok(key) = key_result {
                        if let Some(entry) = table.get_or_create(key) {
                            entry.add("", Value::Table(Table::default()));

                            // assign new container.
                            if let Some(new_table) = entry.find_table_mut("") {
                                table = new_table;
                                key_result = Result::from(token);
                            } else {
                                return Result::from(token);
                            }
                        } else {
                            return Result::from(token);
                        }
                    } else {
                        // start with dot without parent key.
                        return Result::from(token);
                    }
                }
                Kind::EXP => {
                    key_result = string::key_from(source, token);
                }
                Kind::AMPERSAND => {
                    variant_result = string::variants_from(source, token_stream, token);
                    _ = stream::skip_whitespace(token_stream);
                }
                Kind::RSQUARBRACKET => {
                    // end.
                    token_stream.next_token();
                    break;
                }
                _ => {
                    return Result::from(token);
                }
            }
        }

        // process value.
        _ = stream::skip_meanlingless(token_stream);
        if let Ok(key) = key_result {
            if let Some(next_token) = token_stream.peek_token() {
                let value_result = match next_token.kind() {
                    Kind::EXP => Table::from(source, token_stream, next_token, true),
                    Kind::LESSTHAN => Array::from(source, token_stream, next_token),
                    _ => {
                        return Result::from(next_token);
                    }
                };
                let variant_str = match variant_result {
                    Ok(variant) => variant,
                    _ => "",
                };
                match value_result {
                    Ok(value) => {
                        if let Some(entry) = table.get_or_create(key) {
                            entry.add(variant_str, value);
                            return Ok(());
                        } else {
                            return Result::from(next_token);
                        }
                    }
                    _ => return Result::from(next_token),
                }
            } else {
                // empty -- valid or not???
                return Ok(());
            }
        } else {
            return Result::from(start_token);
        }
    }
}

fn on_key_value_expression<'a>(
    source: &'a str,
    token_stream: &mut TokenStream,
    token: &Token,
    container: &mut Table,
) -> Result<()> {
    // 1. key=; variant=; value=;
    let key = match string::key_from(source, token) {
        Ok(k) => k,
        Err(e) => return Err(e),
    };
    token_stream.next_token();
    let mut variant_result: Result<&str> = Result::from(*token);
    let mut value_result: Result<Value> = Result::from(*token);

    // consume current and check next token.
    _ = stream::skip_whitespace(token_stream);
    if let Some(next_token) = token_stream.peek_token() {
        match next_token.kind() {
            Kind::DOT => {
                // here we know it will be a table variant without variant value.
                // table variant: find the entry->find the variant->find the table
                // consume TokenKind::DOT
                token_stream.next_token();
                if let Some(new_token) = token_stream.peek_token() {
                    if let Some(entry) = container.get_or_create(key) {
                        if let Some(new_table) = entry.get_or_create_table("") {
                            on_key_value_expression(source, token_stream, new_token, new_table)?;
                            return Ok(());
                        } else {
                            return Result::from(next_token);
                        }
                    } else {
                        return Result::from(next_token);
                    }
                } else {
                    return Result::from(next_token);
                }
            }
            Kind::AMPERSAND => {
                if let Some(ampersand_token) = token_stream.next_token() {
                    variant_result = string::variants_from(source, token_stream, ampersand_token);
                    _ = stream::skip_whitespace(token_stream);
                    if let Some(end_token) = token_stream.next_token() {
                        if end_token.kind == Kind::EQUALS {
                            if let Some(next_token) = token_stream.next_token() {
                                value_result = Value::from(source, token_stream, next_token);
                            } else {
                                return Result::from(token);
                            }
                        }
                    } else {
                        return Result::from(token);
                    }
                } else {
                    return Result::from(token);
                }
            }
            Kind::EQUALS => {
                // consume =
                if let Some(equal_token) = token_stream.next_token() {
                    _ = stream::skip_whitespace_and_newline(token_stream);
                    value_result = Value::from(source, token_stream, equal_token);
                } else {
                    return Result::from(token);
                }
            }
            _ => {
                // invalid token.
                return Result::from(token);
            }
        }
    } else {
        return Result::from(token);
    }

    let variant_str = match variant_result {
        Ok(variant) => variant,
        _ => "",
    };

    match value_result {
        Ok(value) => {
            if let Some(entry) = container.get_or_create(key) {
                entry.add(variant_str, value);
                Ok(())
            } else {
                Result::from(token)
            }
        }
        Err(e) => Err(e),
    }
}
