//! Token representation and related types.
//!
//! This module contains the core token structure and its constituent parts:
//! token kinds, source spans, and lexeme strings.

pub mod span;
pub mod tokenkind;
pub mod operators;
pub mod keywords;
pub mod literals;
pub mod delimiters;

/// A single token produced by the lexer.
///
/// A `Token` represents a meaningful unit of source code recognized by the
/// lexical analyzer. It combines the token type with its source location and
/// the original text from the input.
///
/// # Fields
///
/// - `kind`: The [`tokenkind::TokenKind`] classifying this token
/// - `span`: A [`span::Span`] marking its location in the source
/// - `lexeme`: The original text from the source code
///
/// # Example
///
/// ```no_run
/// # use hm_lexer::token::{Token, tokenkind::TokenKind, span::Span};
/// # fn example_token() {
/// let token = Token {
///     kind: TokenKind::IntLiteral(42),
///     span: Span {
///         start: 0,
///         end: 2,
///         line_start: 1,
///         column_start: 1,
///         line_end: 1,
///         column_end: 3,
///     },
///     lexeme: "42".to_string(),
/// };
/// # }
/// ```
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Token {
    /// The type and classification of this token.
    pub kind: tokenkind::TokenKind,
    /// The source location (byte offset and line/column) of this token.
    pub span: span::Span,
    /// The original lexeme (text) from the source code.
    pub lexeme: String,
}

impl Token {
    /// Checks if this token is the end-of-file (EOF) token.
    ///
    /// # Returns
    ///
    /// `true` if this token's kind is `TokenKind::Eof`, otherwise `false`.
    pub fn is_eof(&self) -> bool {
        matches!(self.kind, tokenkind::TokenKind::Eof)
    }
}
