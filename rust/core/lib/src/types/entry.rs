use crate::types::table::Table;
use crate::types::traits::Variants;
use crate::types::value::Value;
use indexmap::map;

#[derive(Clone, PartialEq)]
pub struct VariantEntry {
    // key: whole variant string, Value
    // todo: ignore case: here or when envaluating.
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
        for value in self.data.iter() {
            write!(f, "{:#?}", value)?;
        }
        Ok(())
    }
}

impl VariantEntry {
    pub(crate) fn get_or_create_table(&mut self, variant: &str) -> Option<&mut Table> {
        let result = self.find_table_mut(variant);
        if result.is_none() {
            self.add(variant, Value::Table(Table::default()));
        }
        self.find_table_mut(variant)
    }

    pub(crate) fn add(&mut self, variant: &str, value: Value) -> &mut Self {
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
            Some(value) => value.get_table_mut(),
            _ => None,
        }
    }

    pub(crate) fn find<'a, V>(&self, varaints: &'a V) -> Option<&Value>
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
