use crate::types::entry::{self};
use indexmap::map::{self};

#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    // OrderedHashMap.
    // TODO: ignore case: does serd support visiting ignore case???
    data: map::IndexMap<String, entry::VariantEntry>,
}

impl Default for Table {
    fn default() -> Self {
        Self {
            data: map::IndexMap::new(),
        }
    }
}

impl Table {
    pub(crate) fn get_or_create(&mut self, key: &str) -> Option<&mut entry::VariantEntry> {
        self.data
            .entry(key.to_owned())
            .or_insert(entry::VariantEntry::default());
        self.get_mut(key)
    }

    #[cfg(test)]
    pub(crate) fn get(&self, key: &str) -> Option<&entry::VariantEntry> {
        self.data.get(key)
    }

    pub(crate) fn get_mut(&mut self, key: &str) -> Option<&mut entry::VariantEntry> {
        self.data.get_mut(key)
    }

    pub(crate) fn into_map(&self) -> &map::IndexMap<String, entry::VariantEntry> {
        &self.data
    }
}
