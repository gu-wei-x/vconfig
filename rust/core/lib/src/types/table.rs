#![allow(dead_code)]
use crate::types::entry::{self};
use indexmap::map::{self};

#[derive(Clone, Debug)]
pub struct Table {
    // OrderedHashMap.
    data: map::IndexMap<String, entry::VariantEntry>,
}

impl Default for Table {
    fn default() -> Self {
        Self {
            data: map::IndexMap::new(),
        }
    }
}

// ops.
impl Table {
    pub(crate) fn get_or_create(&mut self, key: &str) -> Option<&mut entry::VariantEntry> {
        self.data
            .entry(key.to_owned())
            .or_insert(entry::VariantEntry::default());
        //self.data.get_mut(key)
        self.get_mut(key)
    }

    pub(crate) fn get_mut(&mut self, key: &str) -> Option<&mut entry::VariantEntry> {
        self.data.get_mut(key)
    }

    pub(crate) fn into_map(&self) -> &map::IndexMap<String, entry::VariantEntry> {
        &self.data
    }
}
