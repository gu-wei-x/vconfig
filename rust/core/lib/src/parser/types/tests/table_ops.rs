#![cfg(test)]

#[test]
fn test_table_ops_logic() {
    use crate::parser::types::{array::Array, table::Table, value::Value};

    let mut root = Table::default();

    // cannot have multiple mut borrow in the same sccope.
    /*let string_entry = table.get_or_create("a").unwrap();
    let array_entry = table.get_or_create("b").unwrap();
    let table_entry = table.get_or_create("c").unwrap();*/

    {
        // simple string value.
        let string_entry = root.get_or_create("a").unwrap();
        string_entry.add_item("v:1", Value::String("value1".to_owned()));
        string_entry.add_item("v:2", Value::String("value2".to_owned()));
    }

    {
        // array of strings.
        let array_entry = root.get_or_create("b").unwrap();
        let mut array1 = Array::default();
        array1.add_item(Value::String("value1".to_owned()));
        array1.add_item(Value::String("value2".to_owned()));

        let mut array2 = Array::default();
        array2.add_item(Value::String("value1".to_owned()));
        array2.add_item(Value::String("value2".to_owned()));

        array_entry.add_item("v:1", Value::Array(array1));
        array_entry.add_item("v:2", Value::Array(array2));
    }

    // table.
    {
        let mut new_table = Table::default();
        let string_entry = new_table.get_or_create("a").unwrap();
        string_entry.add_item("v:1", Value::String("value1".to_owned()));
        string_entry.add_item("v:2", Value::String("value2".to_owned()));

        let table_entry = root.get_or_create("c").unwrap();
        table_entry.add_item("v1:1", Value::Table(new_table));
    }
    println!("{:#?}", root);
}
