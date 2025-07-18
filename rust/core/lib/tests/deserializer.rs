#![cfg(test)]
#![allow(dead_code)]

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct TestConfig {
    /*key1: Option<String>,
    key2: Option<u64>,*/
    key1: String,
    key2: u64,
    //sub_config: Option<SubConfig>,
}

/*#[derive(Debug, Deserialize)]
struct SubConfig {
    key1: Option<String>,
    key2: Option<u64>,
    items: Option<Vec<ItemConfig>>,
}

#[derive(Debug, Deserialize)]
struct ItemConfig {
    key1: Option<String>,
    key2: Option<u64>,
}*/

#[test]
fn test_deserializing_config() {
    let raw_str = r#"
        key1&variant1:v1 = "v1"
        key1 = "v2"
        key2&variant2:v2 = "5"
        key2 = "3"

        #[sub_config]
        #key1 = "sv1"
        #key2 = "80"

        #[sub_config.items]
        #<{key1 = "av1", key2 = "1"}, {key1 = "av2", key2 = "2"}>
    "#;

    let mut variants = HashMap::new();
    variants.insert("variant1".to_string(), "v1".to_string());
    variants.insert("variant2".to_string(), "v2".to_string());
    let decoded: TestConfig = variants::from_str_with_variants(raw_str, &variants).unwrap();
    println!("{decoded:#?}");
    assert_eq!(decoded.key1, "v1".to_owned());
    // assert_eq!(decoded.key2, 5);

    /*let sub = decoded.sub_config.unwrap();
    assert_eq!(sub.key1.unwrap(), "sv1".to_owned());
    assert_eq!(sub.key2.unwrap(), 80);
    assert!(sub.items.is_some());

    let items = &sub.items.unwrap();
    assert_eq!(items[0].key1, Some("av1".to_owned()));
    assert_eq!(items[0].key2.unwrap(), 1);*/
}
