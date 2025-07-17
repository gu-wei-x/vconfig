#![allow(dead_code)]
use crate::types::array::Array;
use crate::types::table::Table;

#[derive(Clone /*, Debug*/)]
pub enum Value {
    // simple value must be string style.
    String(String),
    Array(Array),
    Table(Table),
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(str) => write!(f, "Value({:#?})", str)?,
            Value::Array(array) => write!(f, "Value(<{:#?}>)", array)?,
            Value::Table(table) => write!(f, "Value({:#?})", table)?,
        }
        Ok(())
    }
}

impl Value {
    pub(crate) fn string_value(&mut self) -> Option<&str> {
        // todo: impl
        None
    }

    pub(crate) fn array_mut(&mut self) -> Option<&mut Array> {
        if let Value::Array(array) = self {
            Some(array)
        } else {
            None
        }
    }

    pub(crate) fn table_mut(&mut self) -> Option<&mut Table> {
        if let Value::Table(table) = self {
            Some(table)
        } else {
            None
        }
    }
}
