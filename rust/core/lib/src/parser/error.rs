#![allow(dead_code)]
use std::{borrow::Cow, fmt::Display};

use crate::parser::Token;

#[derive(Clone, Debug)]
pub struct Error<'a, T>
where
    T: Clone + Copy + Display,
{
    pub source: T,
    pub msg: Cow<'a, str>,
}

impl<'a, T> Error<'a, T>
where
    T: Clone + Copy + Display,
{
    pub fn new(source: T) -> Self {
        Self {
            source,
            msg: format!("Unexpected: {}", source).into(),
        }
    }
}

impl serde::ser::StdError for Token {}

impl serde::de::Error for Token {
    fn custom<T>(msg: T) -> Self
    where
        T: core::fmt::Display,
    {
        println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
        println!("{:#}", msg);
        println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
        Token::new(super::tokenizer::token::Kind::UNKNOWN, 0, 0)
    }
}
