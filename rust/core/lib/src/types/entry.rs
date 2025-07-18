#![allow(dead_code)]
use crate::types::table::Table;
use crate::types::traits::Variants;
use crate::types::value::Value;
use indexmap::map;

#[derive(Clone /*, Debug*/)]
pub struct VariantEntry {
    // key: whole variant string, Value
    data: map::IndexMap<Option<String>, Value>,
}

impl Default for VariantEntry {
    fn default() -> Self {
        Self {
            data: map::IndexMap::new(),
        }
    }
}

impl std::fmt::Debug for VariantEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // for (variant, value) in self.data.iter() {
        //     write!(f, "({:#?}:{:#?})", variant, value)?;
        // }
        for value in self.data.iter() {
            write!(f, "{:#?}", value)?;
        }
        Ok(())
    }
}

impl VariantEntry {
    pub(crate) fn add_item(&mut self, variant: &str, value: Value) -> &mut Self {
        let key = if variant.is_empty() {
            None
        } else {
            Some(variant.to_string())
        };
        self.data.entry(key).or_insert(value);
        self
    }

    pub(crate) fn find_table_mut(&mut self, variant: &str) -> Option<&mut Table> {
        let key = if variant.is_empty() {
            None
        } else {
            Some(variant.to_string())
        };
        let entry = self.data.get_mut(&key);
        match entry {
            Some(value) => value.table_mut(),
            _ => None,
        }
    }

    pub(crate) fn find_item<'a, V>(&self, varaints: &'a V) -> Option<&Value>
    where
        V: Variants,
    {
        for (variant, value) in self.data.iter() {
            if varaints.matches(variant) {
                return Some(value);
            }
        }
        None
    }
}
