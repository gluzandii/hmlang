//! A lexer (tokenizer) for the Hummingbird programming language.
//!
//! This crate provides the lexical analysis stage for the Hummingbird compiler,
//! converting raw source code into a stream of tokens. The lexer operates on
//! raw bytes rather than assuming UTF-8 validity, allowing for flexible error handling.
//!
//! # Architecture
//!
//! - [`charstream::CharStream`]: A low-level byte stream with position tracking
//! - [`lexer::Lexer`]: The main tokenizer that consumes input and produces tokens
//! - [`token::Token`]: Represents a single token with kind, span, and lexeme
//! - [`token::tokenkind::TokenKind`]: Enumeration of all possible token types
//! - [`token::span::Span`]: Tracks byte offsets and line/column positions
//! - [`lexerror::LexError`]: Error types that can occur during tokenization
//!
//! # Example
//!
//! ```no_run
//! use hm_lexer::charstream::CharStream;
//! use hm_lexer::lexer::Lexer;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let input = "func add(a: int32, b: int32): int32 { return a + b; }";
//! let stream = CharStream::from_bytes(input.as_bytes())?;
//! let mut lexer = Lexer::new(stream);
//!
//! loop {
//!     let token = lexer.next_token()?;
//!     println!("{:?}", token);
//!     // Process token...
//! }
//! # Ok(())
//! # }
//! ```

/// Character stream for byte-level input processing.
pub mod charstream;

/// Main lexer implementation for tokenization.
pub mod lexer;

/// Error types for lexical analysis.
pub mod lexerror;

/// Token types and related structures.
pub mod token;
