#![cfg(test)]
use crate::types::table::Table;
use crate::types::value::Value;

#[test]
fn test_value_string() {
    let str_value = Value::String("test".to_owned());
    assert_eq!("test", str_value.get_str().unwrap());
}

#[test]
fn test_value_table() {
    let table_value = Value::Table(Table::default());
    assert_eq!(&Table::default(), table_value.get_table().unwrap());

    let mut table_value2 = Value::Table(Table::default());
    let table2 = table_value2.get_table_mut().unwrap();
    assert!(table2.get_or_create("test").is_some());
}
