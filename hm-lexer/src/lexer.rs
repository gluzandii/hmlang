//! The main lexer (tokenizer) implementation for the Hummingbird language.
//!
//! The [`Lexer`] consumes characters from a [`CharStream`] and produces [`Token`]s.
//! It handles keywords, identifiers, literals (strings, characters, numbers), and operators.

use crate::charstream::CharStream;
use crate::lexerror::LexError;
use crate::token::operators::arithmetic::ArithmeticOperator;
use crate::token::operators::assignment::AssignmentOperator;
use crate::token::operators::bitwise::BitwiseOperator;
use crate::token::operators::logical::LogicalOperator;
use crate::token::operators::relational::RelationalOperator;
use crate::token::{span::Span, tokenkind::TokenKind, Token};

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

macro_rules! single_char_token {
    ($lexer:expr, $start_idx:expr, $start_line:expr, $start_col:expr, $kind:expr, $lexeme:expr) => {{
        $lexer.stream.advance();
        let (end_idx, end_line, end_col) = $lexer.stream.current_position();
        let span = Span {
            start: $start_idx,
            end: end_idx,
            line_start: $start_line,
            column_start: $start_col,
            line_end: end_line,
            column_end: end_col,
        };
        Token {
            kind: $kind,
            span,
            lexeme: String::from($lexeme),
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
            b'(' => single_char_token!(
                self,
                start_idx,
                start_line,
                start_col,
                TokenKind::LeftParen,
                "("
            ),
            b')' => single_char_token!(
                self,
                start_idx,
                start_line,
                start_col,
                TokenKind::RightParen,
                ")"
            ),
            b'{' => single_char_token!(
                self,
                start_idx,
                start_line,
                start_col,
                TokenKind::LeftBrace,
                "{"
            ),
            b'}' => single_char_token!(
                self,
                start_idx,
                start_line,
                start_col,
                TokenKind::RightBrace,
                "}"
            ),
            b'[' => single_char_token!(
                self,
                start_idx,
                start_line,
                start_col,
                TokenKind::LeftBracket,
                "["
            ),
            b']' => single_char_token!(
                self,
                start_idx,
                start_line,
                start_col,
                TokenKind::RightBracket,
                "]"
            ),

            // Operators and punctuation
            b':' => {
                if self.stream.peek_n(1) == Some(b':') {
                    // :: scoping operator
                    self.stream.advance_n(2);
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
                        kind: TokenKind::ScopingOperator,
                        span,
                        lexeme: String::from("::"),
                    }
                } else {
                    // : colon
                    single_char_token!(
                        self,
                        start_idx,
                        start_line,
                        start_col,
                        TokenKind::Colon,
                        ":"
                    )
                }
            }
            b';' => single_char_token!(
                self,
                start_idx,
                start_line,
                start_col,
                TokenKind::Semicolon,
                ";"
            ),
            b',' => single_char_token!(
                self,
                start_idx,
                start_line,
                start_col,
                TokenKind::Comma,
                ","
            ),
            b'.' => single_char_token!(self, start_idx, start_line, start_col, TokenKind::Dot, "."),
            b'=' => {
                if self.stream.peek_n(1) == Some(b'=') {
                    // == operator
                    self.stream.advance_n(2);
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
                        kind: TokenKind::RelationalOperator(RelationalOperator::Equal),
                        span,
                        lexeme: String::from("=="),
                    }
                } else {
                    // = assignment operator
                    single_char_token!(
                        self,
                        start_idx,
                        start_line,
                        start_col,
                        TokenKind::AssignmentOperator(AssignmentOperator::Assign),
                        "="
                    )
                }
            }
            b'+' => {
                if self.stream.peek_n(1) == Some(b'=') {
                    // += operator
                    self.stream.advance_n(2);
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
                        kind: TokenKind::AssignmentOperator(AssignmentOperator::AddAssign),
                        span,
                        lexeme: String::from("+="),
                    }
                } else {
                    // + arithmetic operator
                    single_char_token!(
                        self,
                        start_idx,
                        start_line,
                        start_col,
                        TokenKind::ArithmeticOperator(ArithmeticOperator::Plus),
                        "+"
                    )
                }
            }
            b'-' => {
                if self.stream.peek_n(1) == Some(b'=') {
                    // -= operator
                    self.stream.advance_n(2);
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
                        kind: TokenKind::AssignmentOperator(AssignmentOperator::SubtractAssign),
                        span,
                        lexeme: String::from("-="),
                    }
                } else {
                    // - arithmetic operator
                    single_char_token!(
                        self,
                        start_idx,
                        start_line,
                        start_col,
                        TokenKind::ArithmeticOperator(ArithmeticOperator::Minus),
                        "-"
                    )
                }
            }
            b'*' => {
                if self.stream.peek_n(1) == Some(b'=') {
                    // *= operator
                    self.stream.advance_n(2);
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
                        kind: TokenKind::AssignmentOperator(AssignmentOperator::MultiplyAssign),
                        span,
                        lexeme: String::from("*="),
                    }
                } else if self.stream.peek_n(1) == Some(b'*') {
                    // ** operator (exponent)
                    self.stream.advance_n(2);
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
                        kind: TokenKind::ArithmeticOperator(ArithmeticOperator::Exponent),
                        span,
                        lexeme: String::from("**"),
                    }
                } else {
                    // * arithmetic operator
                    single_char_token!(
                        self,
                        start_idx,
                        start_line,
                        start_col,
                        TokenKind::ArithmeticOperator(ArithmeticOperator::Asterisk),
                        "*"
                    )
                }
            }
            b'/' => {
                if self.stream.peek_n(1) == Some(b'=') {
                    // /= operator
                    self.stream.advance_n(2);
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
                        kind: TokenKind::AssignmentOperator(AssignmentOperator::DivideAssign),
                        span,
                        lexeme: String::from("/="),
                    }
                } else {
                    // / arithmetic operator
                    single_char_token!(
                        self,
                        start_idx,
                        start_line,
                        start_col,
                        TokenKind::ArithmeticOperator(ArithmeticOperator::Slash),
                        "/"
                    )
                }
            }
            b'%' => {
                if self.stream.peek_n(1) == Some(b'=') {
                    // %= operator
                    self.stream.advance_n(2);
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
                        kind: TokenKind::AssignmentOperator(AssignmentOperator::ModuloAssign),
                        span,
                        lexeme: String::from("%="),
                    }
                } else {
                    // % arithmetic operator
                    single_char_token!(
                        self,
                        start_idx,
                        start_line,
                        start_col,
                        TokenKind::ArithmeticOperator(ArithmeticOperator::Modulo),
                        "%"
                    )
                }
            }
            b'<' => {
                if self.stream.peek_n(1) == Some(b'=') {
                    // <= operator
                    self.stream.advance_n(2);
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
                        kind: TokenKind::RelationalOperator(RelationalOperator::LessThanOrEqual),
                        span,
                        lexeme: String::from("<="),
                    }
                } else if self.stream.peek_n(1) == Some(b'<') {
                    // << operator (left shift)
                    self.stream.advance_n(2);
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
                        kind: TokenKind::BitwiseOperator(BitwiseOperator::LeftShift),
                        span,
                        lexeme: String::from("<<"),
                    }
                } else {
                    // < operator
                    single_char_token!(
                        self,
                        start_idx,
                        start_line,
                        start_col,
                        TokenKind::RelationalOperator(RelationalOperator::LessThan),
                        "<"
                    )
                }
            }
            b'>' => {
                if self.stream.peek_n(1) == Some(b'=') {
                    // >= operator
                    self.stream.advance_n(2);
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
                        kind: TokenKind::RelationalOperator(RelationalOperator::GreaterThanOrEqual),
                        span,
                        lexeme: String::from(">="),
                    }
                } else if self.stream.peek_n(1) == Some(b'>') {
                    // >> operator (right shift)
                    self.stream.advance_n(2);
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
                        kind: TokenKind::BitwiseOperator(BitwiseOperator::RightShift),
                        span,
                        lexeme: String::from(">>"),
                    }
                } else {
                    // > operator
                    single_char_token!(
                        self,
                        start_idx,
                        start_line,
                        start_col,
                        TokenKind::RelationalOperator(RelationalOperator::GreaterThan),
                        ">"
                    )
                }
            }
            b'!' => {
                if self.stream.peek_n(1) == Some(b'=') {
                    // != operator
                    self.stream.advance_n(2);
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
                        kind: TokenKind::RelationalOperator(RelationalOperator::NotEqual),
                        span,
                        lexeme: String::from("!="),
                    }
                } else {
                    // ! logical NOT operator
                    single_char_token!(
                        self,
                        start_idx,
                        start_line,
                        start_col,
                        TokenKind::LogicalOperator(LogicalOperator::Not),
                        "!"
                    )
                }
            }
            b'&' => {
                if self.stream.peek_n(1) == Some(b'&') {
                    // && operator
                    self.stream.advance_n(2);
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
                        kind: TokenKind::LogicalOperator(LogicalOperator::And),
                        span,
                        lexeme: String::from("&&"),
                    }
                } else {
                    // & bitwise AND operator
                    single_char_token!(
                        self,
                        start_idx,
                        start_line,
                        start_col,
                        TokenKind::BitwiseOperator(BitwiseOperator::And),
                        "&"
                    )
                }
            }
            b'|' => {
                if self.stream.peek_n(1) == Some(b'|') {
                    // || operator
                    self.stream.advance_n(2);
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
                        kind: TokenKind::LogicalOperator(LogicalOperator::Or),
                        span,
                        lexeme: String::from("||"),
                    }
                } else {
                    // | bitwise OR operator
                    single_char_token!(
                        self,
                        start_idx,
                        start_line,
                        start_col,
                        TokenKind::BitwiseOperator(BitwiseOperator::Or),
                        "|"
                    )
                }
            }
            b'^' => single_char_token!(
                self,
                start_idx,
                start_line,
                start_col,
                TokenKind::BitwiseOperator(BitwiseOperator::Xor),
                "^"
            ),
            b'~' => single_char_token!(
                self,
                start_idx,
                start_line,
                start_col,
                TokenKind::BitwiseOperator(BitwiseOperator::Not),
                "~"
            ),
            b'?' => single_char_token!(
                self,
                start_idx,
                start_line,
                start_col,
                TokenKind::QuestionMark,
                "?"
            ),

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
    /// Supports decimal integers and floating point numbers.
    /// The method accumulates digits, optionally a decimal point and more digits,
    /// then attempts to parse them as either an `i64` or `f64`.
    ///
    /// # Returns
    ///
    /// - `Ok(Token)` with `TokenKind::IntLiteral` for integers
    /// - `Ok(Token)` with `TokenKind::FloatLiteral` for floating point numbers
    /// - `Err(LexError::InvalidNumber)` if the number is malformed or out of range
    fn lex_number(&mut self) -> Result<Token, LexError> {
        let (start_idx, start_line, start_col) = self.stream.current_position();

        // Consume initial digits
        let (lex_start, _) = self.stream.consume_while(|b| matches!(b, b'0'..=b'9'));

        // Check for decimal point (floating point number)
        let is_float = if self.stream.peek() == Some(b'.') {
            // Peek ahead to ensure there's a digit after the dot
            // This prevents treating "42." as a float or "42.foo" as starting with a float
            if matches!(self.stream.peek_n(1), Some(b'0'..=b'9')) {
                self.stream.advance(); // consume '.'
                // Consume fractional digits
                self.stream.consume_while(|b| matches!(b, b'0'..=b'9'));
                true
            } else {
                false
            }
        } else {
            false
        };

        let (end_idx, end_line, end_col) = self.stream.current_position();

        // Get the lexeme as a string
        let lexeme_bytes = self.stream.slice(lex_start, end_idx);
        let lexeme = String::from_utf8_lossy(lexeme_bytes).to_string();

        // Parse as integer or float
        let kind = if is_float {
            // Validate the float by parsing it
            match lexeme.parse::<f64>() {
                Ok(f) => TokenKind::FloatLiteral(f),
                Err(_) => {
                    return Err(LexError::InvalidNumber {
                        lexeme,
                        line: start_line,
                        column: start_col,
                    });
                }
            }
        } else {
            // Try to parse as integer
            match lexeme.parse::<i64>() {
                Ok(val) => TokenKind::IntLiteral(val),
                Err(_) => {
                    return Err(LexError::InvalidNumber {
                        lexeme,
                        line: start_line,
                        column: start_col,
                    });
                }
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
