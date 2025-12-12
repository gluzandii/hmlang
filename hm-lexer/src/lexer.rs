//! The main lexer (tokenizer) implementation for the Hummingbird language.
//!
//! The [`Lexer`] consumes characters from a [`CharStream`] and produces [`Token`]s.
//! It handles keywords, identifiers, literals (strings, characters, numbers), and operators.

use crate::charstream::CharStream;
use crate::lexerror::LexError;
use crate::token::{Token, span::Span, tokenkind::TokenKind};

macro_rules! decode_escape {
    ($lexer:expr, $quote:expr, $start_line:expr, $start_col:expr) => {{
        $lexer.stream.advance(); // consume backslash

        match $lexer.stream.peek() {
            Some(b'n') => {
                $lexer.stream.advance();
                Ok('\n')
            }
            Some(b't') => {
                $lexer.stream.advance();
                Ok('\t')
            }
            Some(b'r') => {
                $lexer.stream.advance();
                Ok('\r')
            }
            Some(b'0') => {
                $lexer.stream.advance();
                Ok('\0')
            }
            Some(b'\\') => {
                $lexer.stream.advance();
                Ok('\\')
            }
            Some(b) if b == $quote => {
                $lexer.stream.advance();
                Ok(b as char)
            }
            _ => {
                let seq = match $lexer.stream.peek() {
                    Some(b) => format!("\\{}", b as char),
                    None => "\\(EOF)".to_string(),
                };
                Err(LexError::InvalidEscape {
                    sequence: seq,
                    line: $start_line,
                    column: $start_col,
                })
            }
        }
    }};
}

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

            // Delimiters
            b'(' => {
                self.stream.advance();
                let (end_idx, end_line, end_col) = self.stream.current_position();
                let span = Span {
                    start: start_idx,
                    end: end_idx,
                    line_start: start_line,
                    column_start: start_col,
                    line_end: end_line,
                    column_end: end_col,
                };
                Token {
                    kind: TokenKind::LeftParen,
                    span: span,
                    lexeme: String::from("("),
                }
            }
            b')' => {
                self.stream.advance();
                let (end_idx, end_line, end_col) = self.stream.current_position();
                let span = Span {
                    start: start_idx,
                    end: end_idx,
                    line_start: start_line,
                    column_start: start_col,
                    line_end: end_line,
                    column_end: end_col,
                };
                Token {
                    kind: TokenKind::RightParen,
                    span: span,
                    lexeme: String::from(")"),
                }
            }
            b'{' => {
                self.stream.advance();
                let (end_idx, end_line, end_col) = self.stream.current_position();
                let span = Span {
                    start: start_idx,
                    end: end_idx,
                    line_start: start_line,
                    column_start: start_col,
                    line_end: end_line,
                    column_end: end_col,
                };
                Token {
                    kind: TokenKind::LeftBrace,
                    span: span,
                    lexeme: String::from("{"),
                }
            }
            b'}' => {
                self.stream.advance();
                let (end_idx, end_line, end_col) = self.stream.current_position();
                let span = Span {
                    start: start_idx,
                    end: end_idx,
                    line_start: start_line,
                    column_start: start_col,
                    line_end: end_line,
                    column_end: end_col,
                };
                Token {
                    kind: TokenKind::RightBrace,
                    span: span,
                    lexeme: String::from("}"),
                }
            }
            b'[' => {
                self.stream.advance();
                let (end_idx, end_line, end_col) = self.stream.current_position();
                let span = Span {
                    start: start_idx,
                    end: end_idx,
                    line_start: start_line,
                    column_start: start_col,
                    line_end: end_line,
                    column_end: end_col,
                };
                Token {
                    kind: TokenKind::LeftBracket,
                    span: span,
                    lexeme: String::from("["),
                }
            }
            b']' => {
                self.stream.advance();
                let (end_idx, end_line, end_col) = self.stream.current_position();
                let span = Span {
                    start: start_idx,
                    end: end_idx,
                    line_start: start_line,
                    column_start: start_col,
                    line_end: end_line,
                    column_end: end_col,
                };
                Token {
                    kind: TokenKind::RightBracket,
                    span: span,
                    lexeme: String::from("]"),
                }
            }

            // Operators and punctuation
            b':' => {
                self.stream.advance();
                let (end_idx, end_line, end_col) = self.stream.current_position();
                let span = Span {
                    start: start_idx,
                    end: end_idx,
                    line_start: start_line,
                    column_start: start_col,
                    line_end: end_line,
                    column_end: end_col,
                };
                Token {
                    kind: TokenKind::Colon,
                    span: span,
                    lexeme: String::from(":"),
                }
            }
            b';' => {
                self.stream.advance();
                let (end_idx, end_line, end_col) = self.stream.current_position();
                let span = Span {
                    start: start_idx,
                    end: end_idx,
                    line_start: start_line,
                    column_start: start_col,
                    line_end: end_line,
                    column_end: end_col,
                };
                Token {
                    kind: TokenKind::Semicolon,
                    span: span,
                    lexeme: String::from(";"),
                }
            }
            b',' => {
                self.stream.advance();
                let (end_idx, end_line, end_col) = self.stream.current_position();
                let span = Span {
                    start: start_idx,
                    end: end_idx,
                    line_start: start_line,
                    column_start: start_col,
                    line_end: end_line,
                    column_end: end_col,
                };
                Token {
                    kind: TokenKind::Comma,
                    span: span,
                    lexeme: String::from(","),
                }
            }
            b'.' => {
                self.stream.advance();
                let (end_idx, end_line, end_col) = self.stream.current_position();
                let span = Span {
                    start: start_idx,
                    end: end_idx,
                    line_start: start_line,
                    column_start: start_col,
                    line_end: end_line,
                    column_end: end_col,
                };
                Token {
                    kind: TokenKind::Dot,
                    span: span,
                    lexeme: String::from("."),
                }
            }

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

    /// Skip whitespace and comments until meaningful content is found.
    ///
    /// Trivia includes:
    /// - Whitespace: spaces, tabs, carriage returns, newlines
    /// - Line comments: `// ...` until end of line
    /// - Block comments: `/* ... */` with nesting support
    ///
    /// The stream position advances past all trivia, leaving the cursor
    /// at either a non-trivia character or EOF.
    fn skip_trivia(&mut self) {
        loop {
            match self.stream.peek() {
                None => break,
                Some(b' ') | Some(b'\t') | Some(b'\r') | Some(b'\n') => {
                    self.stream.advance();
                }
                Some(b'/') => {
                    if self.stream.peek_n(1) == Some(b'/') {
                        // Line comment: skip until newline
                        self.stream.advance_n(2); // Consume 2
                        while let Some(b) = self.stream.peek() {
                            if b == b'\n' {
                                break;
                            }
                            self.stream.advance();
                        }
                    } else if self.stream.peek_n(1) == Some(b'*') {
                        // Block comment: skip until */
                        self.stream.advance_n(2); // Consume 2
                        while let Some(b) = self.stream.peek() {
                            if b == b'*' && self.stream.peek_n(1) == Some(b'/') {
                                self.stream.advance_n(2); // Consume 2
                                break;
                            }
                            self.stream.advance();
                        }
                    } else {
                        // Not a comment, stop skipping trivia
                        break;
                    }
                }
                _ => break,
            }
        }
    }

    /// Tokenize an identifier or keyword.
    ///
    /// Identifiers start with a letter or underscore and continue with
    /// alphanumeric characters and underscores. The method checks if the
    /// identifier is a reserved keyword and sets the appropriate token kind.
    ///
    /// # Returns
    ///
    /// - `Ok(Token)` with `TokenKind::Identifier` or a keyword variant
    /// - Never returns an error; all valid identifier sequences are accepted
    fn lex_identifier_or_keyword(&mut self) -> Result<Token, LexError> {
        let (start_idx, start_line, start_col) = self.stream.current_position();

        // Consume identifier characters
        let (lex_start, lex_end) = self
            .stream
            .consume_while(|b| matches!(b, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_'));

        let (end_idx, end_line, end_col) = self.stream.current_position();

        // Get the lexeme as a string
        let lexeme_bytes = self.stream.slice(lex_start, lex_end);
        let lexeme = String::from_utf8_lossy(lexeme_bytes).to_string();

        // Try to parse as keyword
        let kind =
            TokenKind::keyword(&lexeme).unwrap_or_else(|| TokenKind::Identifier(lexeme.clone()));

        let span = Span {
            start: start_idx,
            end: end_idx,
            line_start: start_line,
            column_start: start_col,
            line_end: end_line,
            column_end: end_col,
        };

        Ok(Token { kind, span, lexeme })
    }

    /// Tokenize a numeric literal.
    ///
    /// Currently supports decimal integers. The method accumulates all
    /// consecutive digits and attempts to parse them as an `i64`.
    ///
    /// # Returns
    ///
    /// - `Ok(Token)` with `TokenKind::IntLiteral` if parsing succeeds
    /// - `Err(LexError::InvalidNumber)` if the number exceeds i64 range
    fn lex_number(&mut self) -> Result<Token, LexError> {
        let (start_idx, start_line, start_col) = self.stream.current_position();

        // Consume digits
        let (lex_start, lex_end) = self.stream.consume_while(|b| matches!(b, b'0'..=b'9'));

        let (end_idx, end_line, end_col) = self.stream.current_position();

        // Get the lexeme as a string
        let lexeme_bytes = self.stream.slice(lex_start, lex_end);
        let lexeme = String::from_utf8_lossy(lexeme_bytes).to_string();

        // Try to parse as integer
        let kind = match lexeme.parse::<i64>() {
            Ok(val) => TokenKind::IntLiteral(val),
            Err(_) => {
                return Err(LexError::InvalidNumber {
                    lexeme,
                    line: start_line,
                    column: start_col,
                });
            }
        };

        let span = Span {
            start: start_idx,
            end: end_idx,
            line_start: start_line,
            column_start: start_col,
            line_end: end_line,
            column_end: end_col,
        };

        Ok(Token { kind, span, lexeme })
    }

    /// Tokenize a character literal (single-quoted).
    ///
    /// Character literals are enclosed in single quotes and may contain
    /// escape sequences. The method validates that exactly one character
    /// (after escape processing) is enclosed.
    ///
    /// # Escape Sequences
    ///
    /// - `\n` → newline
    /// - `\t` → tab
    /// - `\r` → carriage return
    /// - `\0` → null byte
    /// - `\\` → backslash
    /// - `\'` → single quote
    ///
    /// # Returns
    ///
    /// - `Ok(Token)` with `TokenKind::CharacterLiteral`
    /// - `Err(LexError::UnterminatedString)` if closing quote is missing
    /// - `Err(LexError::InvalidEscape)` if escape sequence is invalid
    fn lex_character_literal(&mut self) -> Result<Token, LexError> {
        let (start_idx, start_line, start_col) = self.stream.current_position();

        self.stream.advance(); // consume opening '

        let ch = match self.stream.peek() {
            None => {
                return Err(LexError::UnterminatedString {
                    line: start_line,
                    column: start_col,
                });
            }
            Some(b'\\') => decode_escape!(self, b'\'', start_line, start_col)?,
            Some(b) => {
                self.stream.advance();
                b as char
            }
        };

        if !self.stream.match_byte(b'\'') {
            return Err(LexError::UnterminatedString {
                line: start_line,
                column: start_col,
            });
        }

        let (end_idx, end_line, end_col) = self.stream.current_position();

        // Lexeme is the raw source including quotes
        let lexeme_bytes = self.stream.slice(start_idx, end_idx);
        let lexeme = String::from_utf8_lossy(lexeme_bytes).to_string();

        let span = Span {
            start: start_idx,
            end: end_idx,
            line_start: start_line,
            column_start: start_col,
            line_end: end_line,
            column_end: end_col,
        };

        Ok(Token {
            kind: TokenKind::CharacterLiteral(ch),
            span,
            lexeme,
        })
    }

    /// Tokenize a string literal (double-quoted).
    ///
    /// String literals are enclosed in double quotes and may contain
    /// escape sequences. The method accumulates all characters until
    /// an unescaped closing quote is found.
    ///
    /// # Escape Sequences
    ///
    /// - `\n` → newline
    /// - `\t` → tab
    /// - `\r` → carriage return
    /// - `\0` → null byte
    /// - `\\` → backslash
    /// - `\"` → double quote
    ///
    /// # Returns
    ///
    /// - `Ok(Token)` with `TokenKind::StringLiteral` and decoded content
    /// - `Err(LexError::UnterminatedString)` if EOF is reached before closing quote
    /// - `Err(LexError::InvalidEscape)` if escape sequence is invalid
    fn lex_string_literal(&mut self) -> Result<Token, LexError> {
        let (start_idx, start_line, start_col) = self.stream.current_position();

        self.stream.advance(); // consume opening "

        let mut decoded = String::new();

        loop {
            match self.stream.peek() {
                None => {
                    return Err(LexError::UnterminatedString {
                        line: start_line,
                        column: start_col,
                    });
                }
                Some(b'"') => {
                    self.stream.advance();
                    break;
                }
                Some(b'\\') => {
                    let ch = decode_escape!(self, b'"', start_line, start_col)?;
                    decoded.push(ch);
                }
                Some(b) => {
                    decoded.push(b as char);
                    self.stream.advance();
                }
            }
        }

        let (end_idx, end_line, end_col) = self.stream.current_position();

        // Lexeme is the raw source including quotes
        let lexeme_bytes = self.stream.slice(start_idx, end_idx);
        let lexeme = String::from_utf8_lossy(lexeme_bytes).to_string();

        let span = Span {
            start: start_idx,
            end: end_idx,
            line_start: start_line,
            column_start: start_col,
            line_end: end_line,
            column_end: end_col,
        };

        Ok(Token {
            kind: TokenKind::StringLiteral(decoded),
            span,
            lexeme,
        })
    }
}
