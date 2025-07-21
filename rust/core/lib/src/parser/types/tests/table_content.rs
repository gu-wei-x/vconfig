#![cfg(test)]
#[test]
fn test_dot_key() {
    use crate::default::DefaultVariants;
    use crate::parser::types;

    let raw_str = r#"
        p.s1.s2.s3 = "value1"
        p.s1.s21 = "value2"
    "#;
    let result = types::parse_str(raw_str);
    println!("result: {:#?}", result);
    assert!(result.is_ok());
    let table = result.unwrap();
    let p_variants = table.get("p");
    assert!(p_variants.is_some());

    let variants = DefaultVariants::default();
    let p_varaints = p_variants.unwrap();
    let p_value = p_varaints.find(&variants);
    assert!(p_value.is_some());

    let p_value = p_value.unwrap().get_table();
    assert!(p_value.is_some());
    print!("{:#?}", p_value);

    let p_table = p_value.unwrap();
    let s1_variants = p_table.get("s1");
    assert!(s1_variants.is_some());
    let s1_value = s1_variants.unwrap().find(&variants);
    assert!(s1_value.is_some());
    let s1_value = s1_value.unwrap().get_table();
    assert!(s1_value.is_some());

    let s1_table = s1_value.unwrap();
    let s2_variants = s1_table.get("s2");
    assert!(s2_variants.is_some());
    let s2_variants = s2_variants.unwrap().find(&variants);
    assert!(s2_variants.is_some());
    let s2_variants = s2_variants.unwrap().get_table();
    assert!(s2_variants.is_some());
    let s2_table = s2_variants.unwrap();
    print!("{:#?}", s2_table);

    // s3 value.
    let s3_variants = s2_table.get("s3");
    assert!(s3_variants.is_some());
    let s3_value = s3_variants.unwrap().find(&variants);
    assert!(s3_value.is_some());
    let s3_value = s3_value.unwrap();
    assert_eq!(s3_value.get_str(), Some("value1"));

    //s21 value
    let s21_variants = s1_table.get("s21");
    assert!(s21_variants.is_some());
    let s21_value = s21_variants.unwrap().find(&variants);
    assert!(s21_value.is_some());
    let s21_value = s21_value.unwrap();
    assert_eq!(s21_value.get_str(), Some("value2"));
}

#[test]
fn test_dot_keys_in_sub_container() {
    use crate::parser::types;
    let raw_output = r#"Ok(
    Table {
        data: {
            "sub_table": (
                None,
                Value(Table {
                    data: {
                        "keys": (
                            None,
                            Value(Table {
                                data: {
                                    "key2": (
                                        Some(
                                            "v1:1",
                                        ),
                                        Value(Table {
                                            data: {
                                                "key1": (
                                                    None,
                                                    Value("value1"),
                                                ),
                                                "key2": (
                                                    None,
                                                    Value("value2"),
                                                ),
                                            },
                                        }),
                                    )(
                                        Some(
                                            "v1:2",
                                        ),
                                        Value(Table {
                                            data: {
                                                "key1": (
                                                    None,
                                                    Value("value3"),
                                                ),
                                                "key2": (
                                                    None,
                                                    Value("value4"),
                                                ),
                                            },
                                        }),
                                    ),
                                },
                            }),
                        )(
                            Some(
                                "v3:3",
                            ),
                            Value(Table {
                                data: {
                                    "key1": (
                                        None,
                                        Value("value5"),
                                    ),
                                    "key2": (
                                        None,
                                        Value("value6"),
                                    ),
                                },
                            }),
                        ),
                    },
                }),
            ),
        },
    },
)"#;

    let raw_str = r#"
        [sub_table.keys.key2&v1:1]
        key1 = "value1"
        key2 = "value2"
        [sub_table.keys.key2&v1:2]
        key1 = "value3"
        key2 = "value4"
        [sub_table.keys&v3:3]
        key1 = "value5"
        key2 = "value6"
    "#;
    let result = types::parse_str(raw_str);
    let output = format!("{:#?}", result);
    assert_eq!(output, raw_output);
}
