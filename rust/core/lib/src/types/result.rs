#![allow(dead_code)]

use crate::types::error;

pub type Result<T> = core::result::Result<T, error::Error>;
