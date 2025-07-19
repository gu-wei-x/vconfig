# variants

variants is a crate to deserilize Rust data based on variant context.

```rust
#![allow(dead_code)]
use std::collections::HashMap;
use std::error::Error;
use variants::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "variants::serde")]
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
```

## Documentation
  * [Doc](./docs/todo.txt): How to use the variants crate.


## Examples

The [examples](./examples) directory contains crates that show how to use variants. Following commands to run an example

```sh
cd examples/basic
cargo run
```