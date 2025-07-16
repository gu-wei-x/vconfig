#![deny(warnings)]
#![allow(dead_code)]

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct TestConfig {
    global_string_key: Option<String>,
    global_integer_key: Option<u64>,
    sub_config: Option<SubConfig>,
}

#[derive(Debug, Deserialize)]
struct SubConfig {
    sub_string_key: Option<String>,
    sub_integer_key: Option<u64>,
    item_configs: Option<Vec<ItemConfig>>,
}

#[derive(Debug, Deserialize)]
struct ItemConfig {
    item_string_key: Option<String>,
    item_integer_key: Option<u64>,
}

fn main() {
    let vtoml_str = r#"
        global_string_key&variant1:v1 = "test"
        global_string_key = "test"
        global_integer_key&variant2:v2 = 5
        global_integer_key = 5

        [sub_config]
        sub_string_key = "test_sub"
        sub_integer_key = 80

        <sub_config.item_configs>
        <{item_string_key = "test_item1", "item_integer_key" = "1"}, {item_string_key = "test_item2", "item_integer_key" = "2"}>
    "#;

    let mut variants = HashMap::new();
    variants.insert("variant1".to_string(), "v1".to_string());
    variants.insert("variant2".to_string(), "v2".to_string());
    let decoded: TestConfig = variants::from_str(vtoml_str, &variants).unwrap();
    println!("{decoded:#?}");
}
