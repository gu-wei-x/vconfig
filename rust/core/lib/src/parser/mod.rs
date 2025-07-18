pub(crate) mod tokenizer;
pub(crate) mod types;

pub(crate) use types::parse_str;

use crate::parser::tokenizer::token::Token;
use crate::types::error;
use crate::types::result::Result;

impl<T> From<Token> for Result<T> {
    fn from(token: Token) -> Self {
        Err(error::Error::from_token(token))
    }
}

impl<T> From<&Token> for Result<T> {
    fn from(token: &Token) -> Self {
        Err(error::Error::from_token(*token))
    }
}

impl error::Error {
    pub(crate) fn from_parser(str: &str) -> Self {
        error::Error::Parser(str.to_owned())
    }

    pub(crate) fn from_token(token: Token) -> Self {
        error::Error::from_parser(&token.to_string())
    }
}
