#![cfg(test)]
use serde::Deserialize;
use std::collections::HashMap;

#[test]
fn test_deserializing_config_simple() {
    #[derive(Debug, Deserialize)]
    struct Config {
        key1: String,
        key2: u64,
    }
    let raw_str = r#"
        key1&variant1:v1 = "v1"
        key1 = "v2"
        key2&variant2:v2 = "5"
        key2 = "3"
    "#;

    let mut variants = HashMap::new();
    variants.insert("variant1".to_string(), "v1".to_string());
    variants.insert("variant2".to_string(), "v2".to_string());
    let result = variants::from_str_with_variants::<Config>(raw_str, &variants);
    assert!(result.is_ok());

    let config = result.unwrap();
    assert_eq!(config.key1, "v1".to_owned());
    assert_eq!(config.key2, 5);
}

#[test]
fn test_deserializing_config_with_sub_config() {
    #[derive(Debug, Deserialize)]
    struct Config {
        key1: String,
        key2: u64,
        sub_config: SubConfig,
    }

    #[derive(Debug, Deserialize)]
    struct SubConfig {
        skey1: String,
        skey2: u64,
    }

    let raw_str = r#"
        key1&variant1:v1 = "v1"
        key1 = "v2"
        key2&variant2:v2 = "5"
        key2 = "3"

        [sub_config]
        skey1&variant1:v1 = "sv1"
        skey1 = "sv2"
        skey2 = "4"
    "#;

    let mut variants = HashMap::new();
    variants.insert("variant1".to_string(), "v1".to_string());
    variants.insert("variant2".to_string(), "v2".to_string());
    let result = variants::from_str_with_variants::<Config>(raw_str, &variants);
    assert!(result.is_ok());

    let config = result.unwrap();
    assert_eq!(config.key1, "v1".to_owned());
    assert_eq!(config.key2, 5);

    let sub_config = &config.sub_config;
    assert_eq!(sub_config.skey1, "sv1".to_owned());
    assert_eq!(sub_config.skey2, 4);
}

#[test]
fn test_deserializing_config_with_array() {
    #[derive(Debug, Deserialize)]
    struct Config {
        data: Vec<String>,
    }

    let raw_str = r#"
        data = <"v0", "v1", "v2">
    "#;

    let variants = HashMap::new();
    let result = variants::from_str_with_variants::<Config>(raw_str, &variants);

    assert!(result.is_ok());

    let config = result.unwrap();
    assert_eq!(config.data.len(), 3);
    assert_eq!(config.data[0], "v0".to_owned());
    assert_eq!(config.data[1], "v1".to_owned());
    assert_eq!(config.data[2], "v2".to_owned());
}
