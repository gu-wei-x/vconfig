use crate::parser::types::array::Array;
use crate::parser::types::{Result, entry, string};
use crate::{
    parser::{Token, types::value::Value},
    tokenizer::{
        stream::{self, TokenStream},
        token::Kind,
    },
};
use indexmap::map;
use winnow::stream::Stream;

#[derive(Clone, Debug)]
pub struct Table {
    // OrderedHashMap.
    data: map::IndexMap<String, entry::VariantEntry>,
}

impl Default for Table {
    fn default() -> Self {
        Self {
            data: map::IndexMap::new(),
        }
    }
}

// ops.
impl Table {
    pub(crate) fn get_or_create(&mut self, key: &str) -> Option<&mut entry::VariantEntry> {
        self.data
            .entry(key.to_owned())
            .or_insert(entry::VariantEntry::default());
        //self.data.get_mut(key)
        self.get_mut(key)
    }

    pub(crate) fn get_mut(&mut self, key: &str) -> Option<&mut entry::VariantEntry> {
        self.data.get_mut(key)
    }
}

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
                /*TokenKind::LSQUARBRACKET => {
                    Self::on_sub_container(source, token_stream, current_token, &mut table)?;
                }*/
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
                Kind::OTHER => {
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
                        let entry = table.get_or_create(key).unwrap();
                        entry.add_item("", Value::Table(Table::default()));

                        // assign new container.
                        table = entry.find_table_mut("").unwrap();
                        key_result = Result::from(token);
                    } else {
                        // start with dot without parent key.
                        return Result::from(token);
                    }
                }
                Kind::OTHER => {
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
                    Kind::OTHER => Table::from(source, token_stream, next_token, true),
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
                        let entry = table.get_or_create(key).unwrap();
                        entry.add_item(variant_str, value);
                        return Ok(());
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
    let key = string::key_from(source, token).unwrap();
    token_stream.next_token();
    let mut variant_result: Result<&str> = Err(*token);
    let mut value_result: Result<Value> = Err(*token);

    // consume current and check next token.
    _ = stream::skip_whitespace(token_stream);
    if let Some(next_token) = token_stream.peek_token() {
        match next_token.kind() {
            Kind::DOT => {
                // here we know it will be a table varaint without varaint value.
                // table variant: find the entry->find the variant->find the table
                let mut new_table = Table::default();
                let new_token = token_stream.next_token().unwrap(); // consume TokenKind::DOT.
                on_key_value_expression(source, token_stream, new_token, &mut new_table)?;
                let entry = container.get_or_create(key).unwrap();
                entry.add_item("", Value::Table(new_table));
                return Ok(());
            }
            Kind::AMPERSAND => {
                let ampersand_token = token_stream.next_token().unwrap();
                variant_result = string::variants_from(source, token_stream, ampersand_token);
                _ = stream::skip_whitespace(token_stream);
                if let Some(end_token) = token_stream.next_token() {
                    if end_token.kind == Kind::EQUALS {
                        let next_token = token_stream.next_token().unwrap();
                        value_result = Value::from(source, token_stream, next_token);
                    }
                } else {
                    // no value
                    return Result::from(token);
                }
            }
            Kind::EQUALS => {
                // consume =
                let equal_token = token_stream.next_token().unwrap();
                _ = stream::skip_whitespace_and_newline(token_stream);
                value_result = Value::from(source, token_stream, equal_token);
            }
            _ => {
                // invlid token.
                return Result::from(token);
            }
        }
    } else {
        // key without value
        return Result::from(token);
    }

    let variant_str = match variant_result {
        Ok(variant) => variant,
        _ => "",
    };

    match value_result {
        Ok(value) => {
            let entry = container.get_or_create(key).unwrap();
            entry.add_item(variant_str, value);
            Ok(())
        }
        _ => Result::from(token),
    }
}
