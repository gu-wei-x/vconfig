#![allow(dead_code)]
use std::{borrow::Cow, fmt::Display};

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
