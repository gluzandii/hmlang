//! Delimiter tokenization logic.
//!
//! This module handles tokenizing delimiters like parentheses, braces,
//! brackets, and punctuation marks.

use super::token_builder::TokenBuilder;
use crate::charstream::CharStream;
use crate::token::operators::SpecialOps;
use crate::token::{delimiters::Delimiters, tokenkind::TokenKind, Token};

/// Tokenize a delimiter based on the byte character.
///
/// # Arguments
///
/// * `stream` - The character stream
/// * `byte` - The delimiter byte character
///
/// # Returns
///
/// The tokenized delimiter
pub fn lex_delimiter(stream: &mut CharStream, byte: u8) -> Token {
    let builder = TokenBuilder::new(stream);
    let d = |kind| TokenKind::Delimiter(kind);

    match byte {
        b'(' => builder.single_char_token(d(Delimiters::LeftParen), "("),
        b')' => builder.single_char_token(d(Delimiters::RightParen), ")"),
        b'{' => builder.single_char_token(d(Delimiters::LeftBrace), "{"),
        b'}' => builder.single_char_token(d(Delimiters::RightBrace), "}"),
        b'[' => builder.single_char_token(d(Delimiters::LeftBracket), "["),
        b']' => builder.single_char_token(d(Delimiters::RightBracket), "]"),
        b';' => builder.single_char_token(d(Delimiters::Semicolon), ";"),
        b',' => builder.single_char_token(d(Delimiters::Comma), ","),
        b'.' => builder.single_char_token(d(Delimiters::Dot), "."),
        b'?' => builder.single_char_token(d(Delimiters::QuestionMark), "?"),
        _ => unreachable!("Invalid delimiter character reached, {}. This shouldn't be possible please debug.", byte),
    }
}

/// Tokenize `:` (colon) or `::` (scope resolution).
///
/// # Returns
///
/// - `::` → `TokenKind::ScopingOperator`
/// - `:` → `TokenKind::Colon`
pub fn lex_colon(stream: &mut CharStream) -> Token {
    let is_scope = stream.peek_n(1) == Some(b':');
    let builder = TokenBuilder::new(stream);
    if is_scope {
        builder.multi_char_token(2, TokenKind::SpecialOperator(SpecialOps::ScopingOperator), "::")
    } else {
        builder.single_char_token(TokenKind::Delimiter(Delimiters::Colon), ":")
    }
}
