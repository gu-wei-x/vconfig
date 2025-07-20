#![cfg(test)]

use crate::parser::tokenizer::token::Kind;
use crate::parser::tokenizer::token::Token;
use crate::parser::types::string;

#[test]
fn test_from() {
    let source = r#""test""#;
    let token = Token::new(Kind::STRING, 0, source.len());
    let result = string::from(source, &token);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), source.trim_matches(&['"', '\'']));
}

#[test]
fn test_key_from() {
    let source = r#"test"#;
    let token = Token::new(Kind::EXP, 0, source.len());
    let result = string::key_from(source, &token);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), source);
}

#[test]
fn test_key_from_invalid() {
    let source = r#"test"#;
    let token = Token::new(Kind::STRING, 0, source.len());
    let result = string::key_from(source, &token);
    assert!(result.is_err());
}
