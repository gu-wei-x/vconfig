#![cfg(test)]

use crate::types::entry::VariantEntry;
use crate::types::value::Value;
use std::collections::HashMap;

#[test]
fn test_variant_entry_table() {
    let mut variant_entry = VariantEntry::default();
    variant_entry.get_or_create_table("");

    let variants: HashMap<String, String> = HashMap::new();
    let variant_table_value2 = variant_entry.find(&variants);
    assert!(variant_table_value2.is_some());
    assert!(variant_table_value2.unwrap().get_table().is_some());
}

#[test]
fn test_variant_entry_value() {
    let mut variant_entry = VariantEntry::default();
    variant_entry.add("", Value::String("test".to_owned()));

    let variants: HashMap<String, String> = HashMap::new();
    let variant_value = variant_entry.find(&variants);
    assert!(variant_value.is_some());
    assert_eq!(variant_value.unwrap(), &Value::String("test".to_owned()));
}
