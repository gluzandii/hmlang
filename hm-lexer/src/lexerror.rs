//! Error types for lexical analysis.
//!
//! This module defines all possible errors that can occur during the
//! tokenization process, with detailed location information for error reporting.

use thiserror::Error;

/// Errors that can occur during lexical analysis.
///
/// All errors include line and column information to help with debugging
/// and user-friendly error reporting.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum LexError {
    /// Unexpected character at the given position.
    #[error("Unexpected character '{ch}' at line {line}, column {column}")]
    UnexpectedCharacter {
        /// The unexpected character encountered
        ch: char,
        /// Line number where the error occurred
        line: usize,
        /// Column number where the error occurred
        column: usize,
    },

    /// Unterminated string literal.
    #[error("Unterminated string literal at line {line}, column {column}")]
    UnterminatedString {
        /// Line number where the string started
        line: usize,
        /// Column number where the string started
        column: usize,
    },

    /// Invalid escape sequence in a string.
    #[error("Invalid escape sequence '{sequence}' at line {line}, column {column}")]
    InvalidEscape {
        /// The invalid escape sequence text
        sequence: String,
        /// Line number where the escape sequence was found
        line: usize,
        /// Column number where the escape sequence was found
        column: usize,
    },

    /// Invalid number format.
    #[error("Invalid number format '{lexeme}' at line {line}, column {column}")]
    InvalidNumber {
        /// The malformed number lexeme
        lexeme: String,
        /// Line number where the number started
        line: usize,
        /// Column number where the number started
        column: usize,
    },

    /// Unexpected end of file.
    #[error("Unexpected end of file at line {line}, column {column}")]
    UnexpectedEof {
        /// Line number where EOF was encountered
        line: usize,
        /// Column number where EOF was encountered
        column: usize,
    },

    /// Invalid UTF-8 sequence encountered.
    #[error("Invalid UTF-8 sequence at line {line}, column {column}")]
    InvalidUtf8 {
        /// Line number where the invalid UTF-8 was found
        line: usize,
        /// Column number where the invalid UTF-8 was found
        column: usize,
    },

    /// Empty input provided.
    #[error("Cannot create CharStream from empty input")]
    EmptyInput,

    /// Input too large to process.
    #[error("Input too large to process: {size} bytes")]
    InputTooLarge {
        /// The size of the input in bytes
        size: usize,
    },
}
