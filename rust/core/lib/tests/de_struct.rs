#![cfg(test)]
use variants::default::DefaultVariants;
use variants::serde::Deserialize;

#[test]
fn test_de_struct_of_string_values() {
    #[derive(Debug, Deserialize)]
    #[serde(crate = "variants::serde")]
    struct Config {
        key0: String,
        key1: String,
    }

    let raw_str = r#"
        key0&v1:1 = "v0"
        key0 = "v1"
        key1&v1:1 = "v2"
        key1 = "v3"
    "#;

    let mut variants = DefaultVariants::default();
    _ = variants.add("v1", "1");
    let result = variants::from_str_with_variants::<Config, _>(raw_str, &variants);
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(&config.key0, "v0");
    assert_eq!(&config.key1, "v2");

    let result =
        variants::from_str_with_variants::<Config, _>(raw_str, &DefaultVariants::default());
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(&config.key0, "v1");
    assert_eq!(&config.key1, "v3");
}

#[test]
fn test_de_struct_in_subcontainer() {
    #[derive(Debug, Deserialize)]
    struct Config {
        key0: String,
        key1: u64,
        sub_config: SubConfig,
    }

    #[derive(Debug, Deserialize)]
    struct SubConfig {
        key0: String,
        key1: u64,
    }

    let raw_str = r#"
        key0&v1:1 = "v0"
        key0 = "v1"
        key1&v2:1 = "0"
        key1 = "1"

        [sub_config]
        key0&v1:1 = "sv0"
        key0 = "sv1"
        key1&v2:1 = "0"
        key1 = "1"
    "#;

    let mut variants = DefaultVariants::default();
    _ = variants.add("v1", "1");
    _ = variants.add("v2", "1");
    let result = variants::from_str_with_variants::<Config, _>(raw_str, &variants);
    assert!(result.is_ok());

    let config = result.unwrap();
    assert_eq!(&config.key0, "v0");
    assert_eq!(config.key1, 0);

    let sub_config = &config.sub_config;
    assert_eq!(sub_config.key0, "sv0");
    assert_eq!(sub_config.key1, 0);

    let result =
        variants::from_str_with_variants::<Config, _>(raw_str, &DefaultVariants::default());
    assert!(result.is_ok());

    let config = result.unwrap();
    assert_eq!(&config.key0, "v1");
    assert_eq!(config.key1, 1);

    let sub_config = &config.sub_config;
    assert_eq!(sub_config.key0, "sv1");
    assert_eq!(sub_config.key1, 1);
}

#[test]
fn test_de_struct_with_dotkeys_and_variants_in_subcontainer() {
    #[derive(Debug, Deserialize)]
    #[serde(crate = "variants::serde")]
    struct Config {
        data: SubConfig,
    }

    #[derive(Debug, Deserialize)]
    #[serde(crate = "variants::serde")]
    struct SubConfig {
        key: String,
    }

    let raw_str = r#"
        [data&v1:1]
        key = "v0"
        [data]
        key = "v1"
    "#;

    let mut variants = DefaultVariants::default();
    _ = variants.add("v1", "1");
    let result = variants::from_str_with_variants::<Config, _>(raw_str, &variants);
    assert!(result.is_ok());

    let config = result.unwrap();
    assert_eq!(&config.data.key, "v0");

    let result =
        variants::from_str_with_variants::<Config, _>(raw_str, &DefaultVariants::default());
    assert!(result.is_ok());

    let config = result.unwrap();
    assert_eq!(&config.data.key, "v1");
}
