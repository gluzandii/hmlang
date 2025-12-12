use thiserror::Error;

/// Errors that can occur during lexical analysis.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum LexError {
    /// Unexpected character at the given position.
    #[error("Unexpected character '{ch}' at line {line}, column {column}")]
    UnexpectedCharacter {
        ch: char,
        line: usize,
        column: usize,
    },

    /// Unterminated string literal.
    #[error("Unterminated string literal at line {line}, column {column}")]
    UnterminatedString { line: usize, column: usize },

    /// Invalid escape sequence in a string.
    #[error("Invalid escape sequence '{sequence}' at line {line}, column {column}")]
    InvalidEscape {
        sequence: String,
        line: usize,
        column: usize,
    },

    /// Invalid number format.
    #[error("Invalid number format '{lexeme}' at line {line}, column {column}")]
    InvalidNumber {
        lexeme: String,
        line: usize,
        column: usize,
    },

    /// Unexpected end of file.
    #[error("Unexpected end of file at line {line}, column {column}")]
    UnexpectedEof { line: usize, column: usize },

    /// Invalid UTF-8 sequence encountered.
    #[error("Invalid UTF-8 sequence at line {line}, column {column}")]
    InvalidUtf8 { line: usize, column: usize },

    /// Empty input provided.
    #[error("Cannot create CharStream from empty input")]
    EmptyInput,

    /// Input too large to process.
    #[error("Input too large to process: {size} bytes")]
    InputTooLarge { size: usize },
}
