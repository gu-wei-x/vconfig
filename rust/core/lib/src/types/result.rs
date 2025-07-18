#![allow(dead_code)]

use crate::{parser::Token, types::error};

pub type Result<T> = core::result::Result<T, error::Error>;

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
