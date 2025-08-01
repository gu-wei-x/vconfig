#![cfg(test)]
use vconfig::default::DefaultVariants;
use vconfig::serde::Deserialize;

#[test]
fn test_de_array_of_string_arrays() {
    #[derive(Debug, Deserialize)]
    #[serde(crate = "vconfig::serde")]
    struct Config {
        data: Vec<Vec<String>>,
    }

    let raw_str = r#"
        data = <<"v0", "v1", "v2">,<"v3", "v4", "v5">>
    "#;

    let variants = DefaultVariants::default();
    let result = vconfig::de::from_str::<Config, _>(raw_str, &variants);

    assert!(result.is_ok());

    let config = result.unwrap();
    assert_eq!(config.data.len(), 2);
    assert_eq!(&config.data[0][0], "v0");
    assert_eq!(&config.data[0][1], "v1");
    assert_eq!(&config.data[0][2], "v2");

    assert_eq!(&config.data[1][0], "v3");
    assert_eq!(&config.data[1][1], "v4");
    assert_eq!(&config.data[1][2], "v5");
}

#[test]
fn test_de_array_of_option_string_arrays() {
    #[derive(Debug, Deserialize)]
    #[serde(crate = "vconfig::serde")]
    struct Config {
        data: Option<Vec<Vec<String>>>,
    }

    let raw_str = r#"
        data = <<"v0", "v1", "v2">,<"v3", "v4", "v5">>
    "#;

    let variants = DefaultVariants::default();
    let result = vconfig::de::from_str::<Config, _>(raw_str, &variants);
    assert!(result.is_ok());

    let config = result.unwrap();
    let data = config.data.unwrap();
    assert_eq!(data.len(), 2);
    assert_eq!(&data[0][0], "v0");
    assert_eq!(&data[0][1], "v1");
    assert_eq!(&data[0][2], "v2");

    assert_eq!(&data[1][0], "v3");
    assert_eq!(&data[1][1], "v4");
    assert_eq!(&data[1][2], "v5");
}

#[test]
fn test_de_array_of_option_string_arrays_none() {
    #[derive(Debug, Deserialize)]
    #[serde(crate = "vconfig::serde")]
    struct Config {
        data: Option<Vec<Vec<String>>>,
    }

    let raw_str = r#"
    "#;

    let variants = DefaultVariants::default();
    let result = vconfig::de::from_str::<Config, _>(raw_str, &variants);
    assert!(result.is_ok());

    let config = result.unwrap();
    assert_eq!(config.data, None);
}

#[test]
fn test_de_array_of_strings() {
    #[derive(Debug, Deserialize)]
    #[serde(crate = "vconfig::serde")]
    struct Config {
        data: Vec<String>,
    }

    let raw_str = r#"
        data = <"v0", "v1", "v2">
    "#;

    let variants = DefaultVariants::default();
    let result = vconfig::de::from_str::<Config, _>(raw_str, &variants);

    assert!(result.is_ok());

    let config = result.unwrap();
    assert_eq!(config.data.len(), 3);
    assert_eq!(&config.data[0], "v0");
    assert_eq!(&config.data[1], "v1");
    assert_eq!(&config.data[2], "v2");
}

#[test]
fn test_de_array_of_strings_in_subcontainer() {
    #[derive(Debug, Deserialize)]
    #[serde(crate = "vconfig::serde")]
    struct Config {
        data: Vec<String>,
    }

    let raw_str = r#"
        [data]
        <"v0", "v1", "v2">
    "#;

    let variants = DefaultVariants::default();
    let result = vconfig::de::from_str::<Config, _>(raw_str, &variants);

    assert!(result.is_ok());

    let config = result.unwrap();
    assert_eq!(config.data.len(), 3);
    assert_eq!(&config.data[0], "v0");
    assert_eq!(&config.data[1], "v1");
    assert_eq!(&config.data[2], "v2");
}

#[test]
fn test_de_array_of_strings_with_variants() {
    #[derive(Debug, Deserialize)]
    #[serde(crate = "vconfig::serde")]
    struct Config {
        data: Vec<String>,
    }

    let raw_str = r#"
        data&v1:1 = <"v0", "v1", "v2">
        data = <"v3", "v4", "v5">
    "#;

    let mut variants = DefaultVariants::default();
    _ = variants.add("v1", "1");
    let result = vconfig::de::from_str::<Config, _>(raw_str, &variants);

    assert!(result.is_ok());

    let config = result.unwrap();
    assert_eq!(config.data.len(), 3);
    assert_eq!(&config.data[0], "v0");
    assert_eq!(&config.data[1], "v1");
    assert_eq!(&config.data[2], "v2");

    // empty
    let result = vconfig::de::from_str::<Config, _>(raw_str, &DefaultVariants::default());
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.data.len(), 3);
    assert_eq!(&config.data[0], "v3");
    assert_eq!(&config.data[1], "v4");
    assert_eq!(&config.data[2], "v5");
}

#[test]
fn test_de_array_of_strings_with_variants_in_subcontainer() {
    #[derive(Debug, Deserialize)]
    #[serde(crate = "vconfig::serde")]
    struct Config {
        data: Vec<String>,
    }

    let raw_str = r#"
        [data&v1:1]
        <"v0", "v1", "v2">
        [data]
        <"v3", "v4", "v5">
    "#;

    let mut variants = DefaultVariants::default();
    _ = variants.add("v1", "1");
    let result = vconfig::de::from_str::<Config, _>(raw_str, &variants);

    assert!(result.is_ok());

    let config = result.unwrap();
    assert_eq!(config.data.len(), 3);
    assert_eq!(&config.data[0], "v0");
    assert_eq!(&config.data[1], "v1");
    assert_eq!(&config.data[2], "v2");

    // empty
    let result = vconfig::de::from_str::<Config, _>(raw_str, &DefaultVariants::default());
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.data.len(), 3);
    assert_eq!(&config.data[0], "v3");
    assert_eq!(&config.data[1], "v4");
    assert_eq!(&config.data[2], "v5");
}

#[test]
fn test_de_array_of_struct_arrays() {
    #[derive(Debug, Deserialize)]
    #[serde(crate = "vconfig::serde")]
    struct Config {
        data: Vec<SubConfig>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(crate = "vconfig::serde")]
    struct SubConfig {
        key: String,
    }

    let raw_str = r#"
        data = <{key = "v0"}, {key = "v1"}>
    "#;

    let variants = DefaultVariants::default();
    let result = vconfig::de::from_str::<Config, _>(raw_str, &variants);

    assert!(result.is_ok());

    let config = result.unwrap();
    assert_eq!(config.data.len(), 2);
    assert_eq!(&config.data[0].key, "v0");
    assert_eq!(&config.data[1].key, "v1");
}

#[test]
fn test_de_array_of_struct_arrays_with_variants() {
    #[derive(Debug, Deserialize)]
    #[serde(crate = "vconfig::serde")]
    struct Config {
        data: Vec<SubConfig>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(crate = "vconfig::serde")]
    struct SubConfig {
        key: String,
    }

    let raw_str = r#"
        data&v1:1 = <{key = "v0"}, {key = "v1"}>
        data = <{key = "v3"}, {key = "v4"}>
    "#;

    let mut variants = DefaultVariants::default();
    _ = variants.add("v1", "1");
    let result = vconfig::de::from_str::<Config, _>(raw_str, &variants);

    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.data.len(), 2);
    assert_eq!(&config.data[0].key, "v0");
    assert_eq!(&config.data[1].key, "v1");

    let result = vconfig::de::from_str::<Config, _>(raw_str, &DefaultVariants::default());
    assert!(result.is_ok());
    let config = result.unwrap();
    assert_eq!(config.data.len(), 2);
    assert_eq!(&config.data[0].key, "v3");
    assert_eq!(&config.data[1].key, "v4");
}

#[test]
fn test_de_array_with_dotkeys_and_variants() {
    #[derive(Debug, Deserialize)]
    #[serde(crate = "vconfig::serde")]
    struct Config {
        data: SubConfig,
    }

    #[derive(Debug, Deserialize)]
    #[serde(crate = "vconfig::serde")]
    struct SubConfig {
        key: Vec<String>,
    }

    let raw_str = r#"
        data.key&v1:1 = <"v0", "v1">
        data.key = <"v2", "v3">
    "#;

    let mut variants = DefaultVariants::default();
    _ = variants.add("v1", "1");
    let result = vconfig::de::from_str::<Config, _>(raw_str, &variants);
    assert!(result.is_ok());

    let config = result.unwrap();
    assert_eq!(&config.data.key[0], "v0");
    assert_eq!(&config.data.key[1], "v1");

    let result = vconfig::de::from_str::<Config, _>(raw_str, &DefaultVariants::default());
    assert!(result.is_ok());

    let config = result.unwrap();
    assert_eq!(&config.data.key[0], "v2");
    assert_eq!(&config.data.key[1], "v3");
}

#[test]
fn test_de_array_with_dotkeys_and_variants_in_subcontainer() {
    #[derive(Debug, Deserialize)]
    #[serde(crate = "vconfig::serde")]
    struct Config {
        data: SubConfig,
    }

    #[derive(Debug, Deserialize)]
    #[serde(crate = "vconfig::serde")]
    struct SubConfig {
        key: Vec<String>,
    }

    let raw_str = r#"
        [data.key&v1:1]
        <"v0", "v1">
        [data.key]
        <"v2", "v3">
    "#;

    let mut variants = DefaultVariants::default();
    _ = variants.add("v1", "1");
    let result = vconfig::de::from_str::<Config, _>(raw_str, &variants);
    assert!(result.is_ok());

    let config = result.unwrap();
    assert_eq!(&config.data.key[0], "v0");
    assert_eq!(&config.data.key[1], "v1");

    let result = vconfig::de::from_str::<Config, _>(raw_str, &DefaultVariants::default());
    assert!(result.is_ok());

    let config = result.unwrap();
    assert_eq!(&config.data.key[0], "v2");
    assert_eq!(&config.data.key[1], "v3");
}
