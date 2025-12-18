//! Helper utilities for building tokens with proper span tracking.
//!
//! The [`TokenBuilder`] eliminates repetitive span construction code
//! by capturing the starting position and providing convenient methods
//! for single and multi-character tokens.

use crate::charstream::CharStream;
use crate::token::{span::Span, tokenkind::TokenKind, Token};

/// A helper for constructing tokens with automatic span tracking.
///
/// This struct captures the starting position when created and provides
/// methods to build tokens while automatically calculating the ending position.
pub struct TokenBuilder<'a> {
    stream: &'a mut CharStream,
    start_idx: usize,
    start_line: usize,
    start_col: usize,
}

impl<'a> TokenBuilder<'a> {
    /// Create a new token builder, capturing the current stream position as the start.
    pub fn new(stream: &'a mut CharStream) -> Self {
        let (start_idx, start_line, start_col) = stream.current_position();
        Self {
            stream,
            start_idx,
            start_line,
            start_col,
        }
    }

    /// Build a single-character token, advancing the stream by 1 position.
    ///
    /// # Arguments
    ///
    /// * `kind` - The token kind
    /// * `lexeme` - The lexeme string (should be 1 character)
    ///
    /// # Returns
    ///
    /// A complete [`Token`] with proper span information
    #[inline]
    pub fn single_char_token(self, kind: TokenKind, lexeme: &str) -> Token {
        self.multi_char_token(1, kind, lexeme)
    }

    /// Build a multi-character token, advancing the stream by `n` positions.
    ///
    /// # Arguments
    ///
    /// * `chars` - Number of characters to advance
    /// * `kind` - The token kind
    /// * `lexeme` - The lexeme string
    ///
    /// # Returns
    ///
    /// A complete [`Token`] with proper span information
    pub fn multi_char_token(self, chars: usize, kind: TokenKind, lexeme: &str) -> Token {
        self.stream.advance_n(chars);
        let (end_idx, end_line, end_col) = self.stream.current_position();
        Token {
            kind,
            span: Span {
                start: self.start_idx,
                end: end_idx,
                line_start: self.start_line,
                column_start: self.start_col,
                line_end: end_line,
                column_end: end_col,
            },
            lexeme: lexeme.to_string(),
        }
    }
}

