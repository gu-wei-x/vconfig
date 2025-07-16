#![allow(dead_code, unused_imports)]
use super::*;
use crate::tokenizer::StrStream;
use std::ops::Range;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u8)]
pub enum TokenKind {
    AMPERSAND = b'&',
    COMMA = b',',
    COMMENT = b'#',
    COLON = b':',
    DOUBLEQUOTEDSTRING = b'"',
    DOT = b'.',
    EQUALS = b'=',
    // array
    LESSTHAN = b'<',
    GREATTHAN = b'>',
    // table
    LSQUARBRACKET = b'[',
    RSQUARBRACKET = b']',
    // inlined table
    LCURLYBRACKET = b'{',
    RCURLYBRACKET = b'}',
    WHITESPACE = b' ',
    NEWLINE = b'\n',
    SINGLEQUOTEDSTRING = b'\'',
    MLDOUBLEQUOTEDSTRING = 1,
    MLSINGLEQUOTEDSTRING,
    OTHER,
    EOF,
    UNKNOWN,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Token {
    pub kind: TokenKind,
    // pub(super) span: Span,
    pub start: usize,
    pub end: usize,
}

impl Token {
    pub fn new(kind: TokenKind, start: usize, end: usize) -> Self {
        Self { kind, start, end }
    }

    #[inline(always)]
    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Token({:?}, {}, {})", self.kind, self.start, self.end)
    }
}

impl Default for Token {
    fn default() -> Self {
        Self {
            kind: TokenKind::UNKNOWN,
            start: 0,
            end: 0,
        }
    }
}

pub fn tokenize(stream: &mut StrStream<'_>) -> Token {
    let Some(peeked_byte) = stream.as_bstr().first() else {
        let start = stream.current_token_start();
        let token = Token::new(TokenKind::EOF, start, start);
        return token;
    };

    let token = match peeked_byte {
        b'&' => tokenize_symbol(stream, TokenKind::AMPERSAND),
        b',' => tokenize_symbol(stream, TokenKind::COMMA),
        b'#' => tokenize_comment(stream),
        b':' => tokenize_symbol(stream, TokenKind::COLON),
        b'"' => tokenize_double_quotated_string(stream),
        b'.' => tokenize_symbol(stream, TokenKind::DOT),
        b'=' => tokenize_symbol(stream, TokenKind::EQUALS),
        b'<' => tokenize_symbol(stream, TokenKind::LESSTHAN),
        b'>' => tokenize_symbol(stream, TokenKind::GREATTHAN),
        b'[' => tokenize_symbol(stream, TokenKind::LSQUARBRACKET),
        b']' => tokenize_symbol(stream, TokenKind::RSQUARBRACKET),
        b'{' => tokenize_symbol(stream, TokenKind::LCURLYBRACKET),
        b'}' => tokenize_symbol(stream, TokenKind::RCURLYBRACKET),
        b' ' => tokenize_whitespace(stream),
        b'\n' => tokenize_newline(stream),
        b'\'' => tokenize_single_quotated_string(stream),
        _ => tokenize_other(stream),
    };

    token
}

fn tokenize_symbol(stream: &mut StrStream<'_>, token_type: TokenKind) -> Token {
    let start = stream.current_token_start();

    // symbol is a single character token.
    let offset = 1;
    stream.next_slice(offset);

    let end = stream.previous_token_end();
    Token::new(token_type, start, end)
}

fn tokenize_whitespace(stream: &mut StrStream<'_>) -> Token {
    let start = stream.current_token_start();
    let offset = stream
        .as_bstr()
        .offset_for(|b| !&(b' ', b'\t').contains_token(b))
        .unwrap_or(stream.eof_offset());
    stream.next_slice(offset);
    let end = stream.previous_token_end();
    Token::new(TokenKind::WHITESPACE, start, end)
}

fn tokenize_comment(stream: &mut StrStream<'_>) -> Token {
    let start = stream.current_token_start();
    let offset = stream
        .as_bytes()
        .find_slice((b'\r', b'\n'))
        .map(|s| s.start)
        .unwrap_or_else(|| stream.eof_offset());
    stream.next_slice(offset);
    let end = stream.previous_token_end();
    Token::new(TokenKind::COMMENT, start, end)
}

fn tokenize_newline(stream: &mut StrStream<'_>) -> Token {
    let start = stream.current_token_start();
    let mut offset = '\r'.len_utf8();
    let has_lf = stream.as_bstr().get(1) == Some(&b'\n');
    if has_lf {
        offset += '\n'.len_utf8();
    }
    stream.next_slice(offset);
    let end = stream.previous_token_end();
    Token::new(TokenKind::NEWLINE, start, end)
}

fn tokenize_other(stream: &mut StrStream<'_>) -> Token {
    let start = stream.current_token_start();
    const TOKEN_START: &[u8] = b"&,;#:\".=[]{} \n'";
    let offset = stream
        .as_bstr()
        .offset_for(|b| TOKEN_START.contains_token(b))
        .unwrap_or_else(|| stream.eof_offset());
    stream.next_slice(offset);
    let end = stream.previous_token_end();
    Token::new(TokenKind::OTHER, start, end)
}

/// string = string-delim *literal-char string-delim
/// string-delim = ' | '''
fn tokenize_single_quotated_string(stream: &mut StrStream<'_>) -> Token {
    let ml_string_delim = "'''";
    let start = stream.current_token_start();
    let mut offset = 1; // skip the opening quote
    let mut token_type = TokenKind::SINGLEQUOTEDSTRING;
    let is_ml = stream.starts_with(ml_string_delim);
    if is_ml {
        offset = 3;
        token_type = TokenKind::MLSINGLEQUOTEDSTRING;
    }

    stream.next_slice(offset);
    let next_offset = match is_ml {
        true => match stream.as_bstr().find_slice(ml_string_delim) {
            Some(range) => range.start,
            None => stream.eof_offset(),
        },
        false => match stream.as_bstr().find_slice((b'\'', b'\n')) {
            Some(range) => range.start,
            None => stream.eof_offset(),
        },
    };

    // skip the closing quotes
    stream.next_slice(next_offset);
    match is_ml {
        true => {
            stream.next_slice(3);
        }
        false => {
            stream.next_slice(1);
        }
    }

    let end = stream.previous_token_end();
    Token::new(token_type, start, end)
}

/// string = string-delim *literal-char string-delim
/// string-delim = " | """"
fn tokenize_double_quotated_string(stream: &mut StrStream<'_>) -> Token {
    let ml_string_delim = "\"\"\"";
    let start = stream.current_token_start();
    let mut offset = 1; // skip the opening quote
    let mut token_type = TokenKind::DOUBLEQUOTEDSTRING;
    let is_ml = stream.starts_with(ml_string_delim);
    if is_ml {
        offset = 3;
        token_type = TokenKind::MLDOUBLEQUOTEDSTRING;
    }

    stream.next_slice(offset);
    let next_offset = match is_ml {
        true => match stream.as_bstr().find_slice(ml_string_delim) {
            Some(range) => range.start,
            None => stream.eof_offset(),
        },
        false => match stream.as_bstr().find_slice((b'"', b'\n')) {
            Some(range) => range.start,
            None => stream.eof_offset(),
        },
    };

    // skip the closing quotes
    stream.next_slice(next_offset);
    match is_ml {
        true => {
            stream.next_slice(3);
        }
        false => {
            stream.next_slice(1);
        }
    }

    let end = stream.previous_token_end();
    Token::new(token_type, start, end)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tokenize_symbol() {
        let input = "[";
        let mut stream = StrStream::new(input);
        let token = token::tokenize(&mut stream);
        assert_eq!(token.kind, token::TokenKind::LSQUARBRACKET);
        assert_eq!(token.start, 0);
        assert_eq!(token.end, 1);
    }
}
