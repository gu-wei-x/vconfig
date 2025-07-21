use crate::types::array::Array;
use crate::types::table::Table;

#[derive(Clone, PartialEq)]
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
    #[cfg(test)]
    pub(crate) fn get_str(&self) -> Option<&str> {
        if let Value::String(str) = self {
            Some(str)
        } else {
            None
        }
    }

    #[cfg(test)]
    pub(crate) fn get_table(&self) -> Option<&Table> {
        if let Value::Table(table) = self {
            Some(table)
        } else {
            None
        }
    }

    pub(crate) fn get_table_mut(&mut self) -> Option<&mut Table> {
        if let Value::Table(table) = self {
            Some(table)
        } else {
            None
        }
    }
}
