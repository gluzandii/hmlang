//! Delimiter tokenization logic.
//!
//! This module handles tokenizing delimiters like parentheses, braces,
//! brackets, and punctuation marks.

use super::token_builder::TokenBuilder;
use crate::charstream::CharStream;
use crate::token::operators::SpecialOperator;
use crate::token::{delimiterkind::DelimiterKind, tokenkind::TokenKind, Token};

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
        b'(' => builder.single_char_token(d(DelimiterKind::LeftParen), "("),
        b')' => builder.single_char_token(d(DelimiterKind::RightParen), ")"),
        b'{' => builder.single_char_token(d(DelimiterKind::LeftBrace), "{"),
        b'}' => builder.single_char_token(d(DelimiterKind::RightBrace), "}"),
        b'[' => builder.single_char_token(d(DelimiterKind::LeftBracket), "["),
        b']' => builder.single_char_token(d(DelimiterKind::RightBracket), "]"),
        b';' => builder.single_char_token(d(DelimiterKind::Semicolon), ";"),
        b',' => builder.single_char_token(d(DelimiterKind::Comma), ","),
        b'.' => builder.single_char_token(d(DelimiterKind::Dot), "."),
        b'?' => builder.single_char_token(d(DelimiterKind::QuestionMark), "?"),
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
        builder.multi_char_token(2, TokenKind::SpecialOperator(SpecialOperator::ScopingOperator), "::")
    } else {
        builder.single_char_token(TokenKind::Delimiter(DelimiterKind::Colon), ":")
    }
}
