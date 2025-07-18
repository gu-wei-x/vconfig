use winnow::stream::Stream as _;

use crate::{
    parser::Token,
    parser::types::Result,
    tokenizer::{
        stream::{self, TokenStream},
        token::Kind,
    },
};

pub(crate) fn from<'a>(source: &'a str, token: &Token) -> Result<&'a str> {
    // todo: validate range.
    let result = &source[token.range().start..token.range().end];
    let new_str = result.trim_matches(&['"', '\'']);
    Ok(new_str)
}

pub(crate) fn key_from<'a>(source: &'a str, token: &Token) -> Result<&'a str> {
    // todo: validate token is other.
    from(source, token)
}

pub(crate) fn variants_from<'a>(
    source: &'a str,
    token_stream: &mut TokenStream,
    token: &Token,
) -> Result<&'a str> {
    // todo: validate token is other.
    // start with &
    let start_token = token_stream.next_token().unwrap(); // consume current.
    if let Some(end_token) = stream::get_next_token_if(token_stream, |k| {
        !vec![
            Kind::WHITESPACE,
            Kind::NEWLINE,
            Kind::EQUALS,
            Kind::GREATTHAN,
            Kind::RSQUARBRACKET,
        ]
        .contains(&k)
    }) {
        let token = Token::new(
            Kind::OTHER,
            start_token.range().start,
            end_token.range().start,
        );
        key_from(source, &token)
    } else {
        Err(*token)
    }
}
