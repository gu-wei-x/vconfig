vconfig
===========================
[<img alt="github" src="https://img.shields.io/badge/github-guweix/vconfig-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/gu-wei-x/vconfig)
[<img alt="crates.io" src="https://img.shields.io/crates/v/vconfig.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/vconfig)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-vconfig-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/vconfig)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/gu-wei-x/vconfig/ci.yml?branch=main&style=for-the-badge" height="20">](https://github.com/gu-wei-x/vconfig/actions?query=branch%3Amain)

vconfig is a crate to deserialize Rust data from TOML-formatted like files/streams based on variant context.

```rust
use std::error::Error;
use vconfig::default::Defaultvariants;
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
    let mut variants = Defaultvariants::default();
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
    let mut variants = Defaultvariants::default();
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