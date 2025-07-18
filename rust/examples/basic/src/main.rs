#![allow(dead_code)]
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Config {
    key1: String,
    key2: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    from_str()?;
    from_file()?;
    Ok(())
}

fn from_str() -> Result<(), Box<dyn Error>> {
    let raw_str = r#"
        key1&variant1:v1 = "v1"
        key1 = "v2"
        key2&variant2:v2 = "5"
        key2 = "3"
    "#;
    let mut variants = HashMap::new();
    variants.insert("variant1".to_string(), "v1".to_string());
    variants.insert("variant2".to_string(), "v2".to_string());
    let result = variants::from_str_with_variants::<Config, _>(raw_str, &variants);
    println!("{:?}", result); // Ok(Config { key1: "v1", key2: 5 })
    Ok(())
}

fn from_file() -> Result<(), Box<dyn Error>> {
    let mut variants = HashMap::new();
    variants.insert("variant1".to_string(), "v1".to_string());
    variants.insert("variant2".to_string(), "v2".to_string());
    let result = variants::from_file_with_variants::<Config, _, _>("basic.toml", &variants);
    println!("{:?}", result); // Ok(Config { key1: "v1", key2: 5 })
    Ok(())
}
