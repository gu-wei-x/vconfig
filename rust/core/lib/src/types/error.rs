#![allow(dead_code)]
use crate::parser::Token;
use std::fmt::Debug;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    String(String),
    Parser(Token),
    De(String),
}

impl Error {
    pub(crate) fn from_token(token: Token) -> Self {
        Error::Parser(token)
    }

    pub(crate) fn from_str(str: &str) -> Self {
        Error::String(str.to_owned())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::String(msg) => {
                write!(f, "Err:({})", msg)
            }

            Error::Parser(token) => {
                write!(f, "Paser Err({:?})", token)
            }

            Error::De(msg) => {
                write!(f, "Deserialization Err({:?})", msg)
            }
        }
    }
}

// for serde::de
impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: core::fmt::Display,
    {
        println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
        println!("{:#}", msg);
        println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
        Self::De("test".to_owned())
    }

    // todo:: more
}

impl serde::de::StdError for Error {
    // todo:: more
}
