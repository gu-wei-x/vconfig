#![cfg(test)]
macro_rules! tokenizer_test_case {
    ($name:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            use crate::parser::tokenizer::Tokenizer;
            use crate::parser::tokenizer::token;

            // Join the input strings to form a single string for tokenization.
            let tokenizer_input: String = $input.join("");
            println!("input sting: {:?}", tokenizer_input);
            let tokenizer = Tokenizer::new(&tokenizer_input);
            let tokens: Vec<token::Token> = tokenizer.into_vec();
            println!("tokens: {:?}", tokens);
            assert_eq!(tokens.len(), $expected.len());
            for (i, token) in tokens.iter().enumerate() {
                // kind.
                assert_eq!(token.kind(), $expected[i]);

                // range.
                match token.kind() {
                    token::Kind::EOF => {
                        assert_eq!(tokenizer_input.len()..tokenizer_input.len(), token.range());
                    }
                    _ => {
                        assert_eq!($input[i], &tokenizer_input[token.range()]);
                    }
                }
            }
        }
    };
}

tokenizer_test_case!(test_tokenizer_eof, vec![""], [token::Kind::EOF]);

tokenizer_test_case!(
    test_tokenizer_symbols,
    vec![
        "&", ",", ":", ".", "=", "<", ">", "[", "]", "{", "}", "\r\n", "\n"
    ],
    vec![
        token::Kind::AMPERSAND,
        token::Kind::COMMA,
        token::Kind::COLON,
        token::Kind::DOT,
        token::Kind::EQUALS,
        token::Kind::LESSTHAN,
        token::Kind::GREATTHAN,
        token::Kind::LSQUARBRACKET,
        token::Kind::RSQUARBRACKET,
        token::Kind::LCURLYBRACKET,
        token::Kind::RCURLYBRACKET,
        token::Kind::NEWLINE,
        token::Kind::NEWLINE,
        token::Kind::EOF
    ]
);

tokenizer_test_case!(
    test_tokenizer_comment,
    vec!["# This is a comment", "\n"],
    vec![token::Kind::COMMENT, token::Kind::NEWLINE, token::Kind::EOF]
);

tokenizer_test_case!(
    test_tokenizer_sl_double_quoted_string,
    vec!["\"This is a double quoted string\"", "\n"],
    vec![token::Kind::STRING, token::Kind::NEWLINE, token::Kind::EOF]
);

tokenizer_test_case!(
    test_tokenizer_ml_double_quoted_string,
    vec![
        "\"\"\"This is a multi-line double quoted string\n\"\"\"",
        "\n"
    ],
    vec![token::Kind::STRING, token::Kind::NEWLINE, token::Kind::EOF]
);

tokenizer_test_case!(
    test_tokenizer_sl_single_quoted_string,
    vec!["'This is a single quoted string'", "\n"],
    vec![token::Kind::STRING, token::Kind::NEWLINE, token::Kind::EOF]
);

tokenizer_test_case!(
    test_tokenizer_ml_single_quoted_string,
    vec!["'''\nThis is a multi-line single quoted string\n'''", "\n"],
    vec![token::Kind::STRING, token::Kind::NEWLINE, token::Kind::EOF]
);

tokenizer_test_case!(
    test_tokenizer_exp_without_value,
    vec!["abc"],
    [token::Kind::EXP, token::Kind::EOF]
);

tokenizer_test_case!(
    test_tokenizer_exp_with_string_value,
    vec!["key", " ", "=", " ", "\"value\""],
    [
        token::Kind::EXP,
        token::Kind::WHITESPACE,
        token::Kind::EQUALS,
        token::Kind::WHITESPACE,
        token::Kind::STRING,
        token::Kind::EOF
    ]
);

tokenizer_test_case!(
    test_tokenizer_exp_with_exp_value,
    vec!["key", " ", "=", " ", "value"],
    [
        token::Kind::EXP,
        token::Kind::WHITESPACE,
        token::Kind::EQUALS,
        token::Kind::WHITESPACE,
        token::Kind::EXP,
        token::Kind::EOF
    ]
);
