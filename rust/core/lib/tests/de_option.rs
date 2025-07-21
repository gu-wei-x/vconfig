#![cfg(test)]
use paste;

macro_rules! de_option_case {
    ($name:ident, $type:ident, $value:expr, $expected:expr, $is_some:expr) => {
        paste::item! {
            #[test]
            fn [<de_test_option_ $name _$type _$is_some>]() {
                use serde::Deserialize;

                #[derive(Debug, Deserialize)]
                struct Config {
                    key: Option<$type>,
                }

                let raw_str = if $is_some {
                    format!(
                        r#"
                           key = "{}"
                        "#, $value)
                } else {
                    r#" "#.to_owned()
                };

                let variants = variants::default::DefaultVariants::default();
                let result = variants::from_str_with_variants::<Config, _>(
                    &raw_str, &variants,
                );

                println!("{:?}", result);
                assert!(result.is_ok());
                let config = result.unwrap();
                match $is_some {
                    true => {
                        assert!(config.key.is_some());
                        assert_eq!(config.key.unwrap(), $expected)
                    }
                    false =>{
                        assert!(config.key.is_none())
                    }
                }

            }
        }
    };
}

de_option_case!(int, u8, "1", 1, true);
de_option_case!(int, u8, "1", 1, false);
de_option_case!(true_has_value, bool, "true", true, true);
de_option_case!(false_has_value, bool, "false", false, true);
de_option_case!(true_without_value, bool, "true", true, false);
de_option_case!(false_without_value, bool, "false", false, false);
