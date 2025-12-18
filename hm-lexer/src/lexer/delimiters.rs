//! Delimiter tokenization logic.
//!
//! This module handles tokenizing delimiters like parentheses, braces,
//! brackets, and punctuation marks.

use crate::charstream::CharStream;
use crate::token::{tokenkind::TokenKind, Token};

use super::token_builder::TokenBuilder;

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

    match byte {
        b'(' => builder.single_char_token(TokenKind::LeftParen, "("),
        b')' => builder.single_char_token(TokenKind::RightParen, ")"),
        b'{' => builder.single_char_token(TokenKind::LeftBrace, "{"),
        b'}' => builder.single_char_token(TokenKind::RightBrace, "}"),
        b'[' => builder.single_char_token(TokenKind::LeftBracket, "["),
        b']' => builder.single_char_token(TokenKind::RightBracket, "]"),
        b';' => builder.single_char_token(TokenKind::Semicolon, ";"),
        b',' => builder.single_char_token(TokenKind::Comma, ","),
        b'.' => builder.single_char_token(TokenKind::Dot, "."),
        b'?' => builder.single_char_token(TokenKind::QuestionMark, "?"),
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
        builder.multi_char_token(2, TokenKind::ScopingOperator, "::")
    } else {
        builder.single_char_token(TokenKind::Colon, ":")
    }
}

