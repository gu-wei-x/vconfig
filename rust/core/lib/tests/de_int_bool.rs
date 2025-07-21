#![cfg(test)]
use paste;

macro_rules! de_test_case {
    ($name:ident, $type:ident, $value:expr, $expected:expr) => {
        paste::item! {
            #[test]
            fn [<de_test_ $name _$type>]() {
                use serde::Deserialize;

                #[derive(Debug, Deserialize)]
                struct Config {
                    key: $type,
                }

                let raw_str = format!(
                    r#"
                       key = "{}"
                    "#,
                    $value
                );

                let variants = variants::default::DefaultVariants::default();
                let result = variants::from_str_with_variants::<Config, _>(
                    &raw_str, &variants,
                );
                println!("{:?}", result);
                assert!(result.is_ok());
                let config = result.unwrap();
                assert_eq!(config.key, $expected);
            }
        }
    };

    ($type:ident, $expected:expr) => {
        paste::item! {
            #[test]
            fn [<de_test_$type>]() {
                use serde::Deserialize;

                #[derive(Debug, Deserialize)]
                struct Config {
                    key: $type,
                }

                let raw_str = format!(
                    r#"
                       key = "{}"
                    "#,
                    stringify!($expected)
                );

                let variants = variants::default::DefaultVariants::default();
                let result = variants::from_str_with_variants::<Config, _>(
                    &raw_str, &variants,
                );
                assert!(result.is_ok());
                let config = result.unwrap();
                assert_eq!(config.key, $expected);
            }
        }
    };
}

// integers.
de_test_case!(i8, 8);
de_test_case!(i16, 16);
de_test_case!(i64, 64);
de_test_case!(i128, 128);
de_test_case!(u8, 8);
de_test_case!(u16, 16);
de_test_case!(u64, 64);
de_test_case!(u128, 128);

// bool: only true|false
// 1|0, yes|no are not supported.
de_test_case!(true, bool, "true", true);
de_test_case!(false, bool, "false", false);

/* might support in future */
/*
de_test_case!(yes, bool, "yes", true);
de_test_case!(one, bool, "1", true);
de_test_case!(zero, bool, "0", false);
de_test_case!(no, bool, "no", false);*/
