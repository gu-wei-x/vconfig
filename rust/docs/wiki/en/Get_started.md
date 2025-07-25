# Getting Started

Let's create and run application with Variants. Rust toolchain is prerquired, create a new Cargo project that depends on
Variants, and then build/run the application.

## Installing Rust

To install the latest version of Rust, we recommend using `rustup`. Install
`rustup` by following the instructions on [its website](https://rustup.rs/).
Once `rustup` is installed, ensure the latest toolchain is installed by running
the command:

```sh
rustup default stable
```

## Hello, world!

Let's write first application leveraging Variants! Start by creating a new binary-based
Cargo project and changing into the new directory:

```sh
cargo new test-variants --bin
cd test-variants
cargo add variants
```
**Note: todo pub the crate.**

Modify `src/main.rs` so that it contains the following code:
```
use std::error::Error;
use variants::default::DefaultVariants;
use variants::serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
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
    let mut variants = DefaultVariants::default();
    _ = variants.add("variant1", "v1");
    _ = variants.add("variant2", "v2");
    let result = variants::de::from_str_with_variants::<Config, _>(raw_str, &variants);
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
    let result = variants::de::from_file_with_variants::<Config, _, _>("basic.toml", &variants);
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
Use command to run and check the output:

```sh
> cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running `test-variants.exe`
Ok(Config { key1: "v1", key2: 5 })
Ok(Config { key1: "v1", key2: 5 })
```

---
### [Next: Use Variants in Active Web apps](./Actix_web.md)