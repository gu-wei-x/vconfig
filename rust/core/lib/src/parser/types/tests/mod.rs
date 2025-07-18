#![cfg(test)]
pub(crate) mod array;
pub(crate) mod table;

#[doc(hidden)]
//#[macro_use]
mod macros {
    #[macro_export]
    macro_rules! parser_test_case {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                use crate::parser::types;

                let result = types::parse_str($input);
                println!("result: {:#?}", result);
                if ($expected) {
                    assert!(
                        result.is_ok(),
                        "Expected parsing to succeed, but it failed."
                    );
                } else {
                    assert!(
                        result.is_err(),
                        "Expected parsing to fail, but it succeeded."
                    );
                }
            }
        };
    }
    pub(crate) use parser_test_case;
}

// use following command to show output:
// cargo test test_name -- --nocapture
