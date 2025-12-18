//! The main lexer (tokenizer) implementation for the Hummingbird language.
//!
//! The [`Lexer`] consumes characters from a [`crate::charstream::CharStream`] and produces [`crate::token::Token`]s.
//! It handles keywords, identifiers, literals (strings, characters, numbers), and operators.

mod delimiters;
mod macros;
mod operators;
mod parsing;
mod token_builder;
mod trivia;

use crate::charstream::CharStream;
use crate::lexerror::LexError;
use crate::token::{span::Span, tokenkind::TokenKind, Token};


/// The main lexer that converts a byte stream into a sequence of tokens.
///
/// `Lexer` is responsible for the lexical analysis phase of compilation.
/// It reads characters from a [`CharStream`], recognizes language constructs
/// (keywords, identifiers, literals, operators), and produces tokens with
/// associated source location information.
///
/// # Features
///
/// - **Keyword recognition**: Automatically identifies language keywords
/// - **Literal parsing**: Handles strings, characters, and numeric literals
/// - **Error reporting**: Provides detailed error messages with line/column info
/// - **Trivia skipping**: Automatically skips whitespace and comments
/// - **Position tracking**: Maintains accurate source location metadata
pub struct Lexer {
    /// The underlying byte stream being tokenized.
    stream: CharStream,
}

impl Lexer {
    /// Create a new lexer from a character stream.
    ///
    /// # Arguments
    ///
    /// * `stream` - A [`CharStream`] positioned at the start of the input
    ///
    /// # Returns
    ///
    /// A new [`Lexer`] instance ready to tokenize the input
    pub fn new(stream: CharStream) -> Self {
        Self { stream }
    }

    /// Extract the next token from the input stream.
    ///
    /// This method skips any trivia (whitespace and comments), then
    /// identifies and returns the next meaningful token. Returns an
    /// `Eof` token when the input is exhausted, or a [`LexError`] if
    /// invalid input is encountered.
    ///
    /// # Returns
    ///
    /// - `Ok(Token)` containing the next token and its metadata
    /// - `Err(LexError)` if an invalid character or malformed literal is encountered
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use hm_lexer::charstream::CharStream;
    /// # use hm_lexer::lexer::Lexer;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut lexer = Lexer::new(CharStream::from_bytes(b"var x = 42")?);
    /// let token = lexer.next_token()?;
    /// println!("First token: {:?}", token);
    /// # Ok(())
    /// # }
    /// ```
    pub fn next_token(&mut self) -> Result<Token, LexError> {
        // Skip trivia (whitespace and comments)
        self.skip_trivia();

        // Capture the start position for the token's span
        let (start_idx, start_line, start_col) = self.stream.current_position();

        // Check for EOF
        if self.stream.is_eof() {
            let span = Span {
                start: start_idx,
                end: start_idx,
                line_start: start_line,
                column_start: start_col,
                line_end: start_line,
                column_end: start_col,
            };
            return Ok(Token {
                kind: TokenKind::Eof,
                span,
                lexeme: String::new(),
            });
        }

        // Peek at the next character and dispatch
        let byte = self.stream.peek().unwrap();

        let token = match byte {
            // Character literals
            b'\'' => self.lex_character_literal()?,

            // String literals
            b'"' => self.lex_string_literal()?,

            // Identifiers and keywords
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.lex_identifier_or_keyword()?,

            // Numeric literals
            b'0'..=b'9' => self.lex_number()?,

            // Delimiters (simple punctuation)
            b'(' | b')' | b'{' | b'}' | b'[' | b']' | b';' | b',' | b'.' | b'?' => {
                delimiters::lex_delimiter(&mut self.stream, byte)
            }

            // Colon (can be : or ::)
            b':' => delimiters::lex_colon(&mut self.stream),

            // Operators (all delegated to operators module)
            b'=' | b'+' | b'-' | b'*' | b'/' | b'%' | b'<' | b'>' | b'!' | b'&' | b'|'
            | b'^' | b'~' => operators::lex_operator(&mut self.stream, byte)?,

            // Unexpected character
            _ => {
                let ch = byte as char;
                return Err(LexError::UnexpectedCharacter {
                    ch,
                    line: start_line,
                    column: start_col,
                });
            }
        };

        Ok(token)
    }
}
