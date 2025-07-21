#![cfg(test)]
use crate::types::array::Array;
use crate::types::table::Table;
use crate::types::value::Value;

#[test]
fn test_array_item_array() {
    let mut array = Array::default();
    array.push(Value::Array(Array::default())).unwrap();
    array.push(Value::Array(Array::default())).unwrap();

    let vec = array.into_vec();
    assert_eq!(vec[0], Value::Array(Array::default()));
    assert_eq!(vec[1], Value::Array(Array::default()));
}

#[test]
fn test_array_item_string() {
    let mut array = Array::default();
    array.push(Value::String("value1".to_owned())).unwrap();
    array.push(Value::String("value2".to_owned())).unwrap();

    let vec = array.into_vec();
    assert_eq!(vec[0], Value::String("value1".to_owned()));
    assert_eq!(vec[1], Value::String("value2".to_owned()));
}

#[test]
fn test_array_item_table() {
    let mut array = Array::default();
    array.push(Value::Table(Table::default())).unwrap();
    array.push(Value::Table(Table::default())).unwrap();

    let vec = array.into_vec();
    assert_eq!(vec[0], Value::Table(Table::default()));
    assert_eq!(vec[1], Value::Table(Table::default()));
}

#[test]
#[should_panic]
fn test_array_item_type() {
    let mut array = Array::default();
    array.push(Value::String("value1".to_owned())).unwrap();
    array.push(Value::Table(Table::default())).unwrap();
}
