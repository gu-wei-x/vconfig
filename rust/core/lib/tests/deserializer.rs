//#![cfg(test)]
//todo: add more
/*#![deny(warnings)]
#![allow(dead_code)]

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct TestConfig {
    key1: Option<String>,
    key2: Option<u64>,
    sub_config: Option<SubConfig>,
}

#[derive(Debug, Deserialize)]
struct SubConfig {
    key1: Option<String>,
    key2: Option<u64>,
    array_config: Option<Vec<ItemConfig>>,
}

#[derive(Debug, Deserialize)]
struct ItemConfig {
    item_key1: Option<String>,
    item_key2: Option<u64>,
}

fn main() {
    let raw_str = r#"
        key1&variant1:v1 = "test"
        key1 = "test"
        key2&variant2:v2 = "5"
        key2 = "5"

        [sub_config]
        key1 = "test_sub"
        key2 = "80"

        [sub_config.array_config]
        <{key1 = "v1", key2 = "1"}, {key1 = "v3", key2 = "2"}>
    "#;

    let mut variants = HashMap::new();
    variants.insert("variant1".to_string(), "v1".to_string());
    variants.insert("variant2".to_string(), "v2".to_string());
    let decoded: TestConfig = variants::from_str(raw_str, &variants).unwrap();
    println!("{decoded:#?}");
}*/
