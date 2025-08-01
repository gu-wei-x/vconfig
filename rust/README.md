# vconfig

vconfig is a crate to deserialize Rust data from TOML-formatted like files/streams based on variant context.

```rust
use std::error::Error;
use vconfig::default::DefaultVariants;
use vconfig::serde::Deserialize;
use vconfig::traits::Variants;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(crate = "vconfig::serde")]
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
    let mut variants = DefaultVariants::default();
    _ = variants.add("variant1", "v1");
    _ = variants.add("variant2", "v2");
    let result = vconfig::de::from_str::<Config, _>(raw_str, &variants);
    println!("{:?}", result); // Ok(Config { key1: "v1", key2: 5 })
    assert_eq!(
        result,
        Ok(Config {
            key1: "v1".to_string(),
            key2: 5
        })
    );
    Ok(())
}

fn from_file() -> Result<(), Box<dyn Error>> {
    let mut variants = DefaultVariants::default();
    _ = variants.add("variant1", "v1");
    _ = variants.add("variant2", "v2");
    let result = vconfig::de::from_file::<Config, _, _>("basic.toml", &variants);
    println!("{:?}", result); // Ok(Config { key1: "v1", key2: 5 })
    assert_eq!(
        result,
        Ok(Config {
            key1: "v1".to_string(),
            key2: 5
        })
    );
    Ok(())
}
```

## Documentation
  * [Guide: Use the vconfig crate](./docs/wiki/en/Introduction.md)


## Examples

The [examples](./examples) directory contains crates that show how to use variants. Following commands to run an example

```sh
cd examples/basic
cargo run
```