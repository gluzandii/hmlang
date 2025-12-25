//! Parsing logic for literals and identifiers.
//!
//! This module contains implementations for parsing various token types
//! including character literals, string literals, identifiers, keywords,
//! and numeric literals (integers and floats).

use crate::decode_escape;
use crate::lexer::Lexer;
use crate::lexerror::LexError;
use crate::token::literalkind::LiteralKind;
use crate::token::span::Span;
use crate::token::tokenkind::TokenKind;
use crate::token::Token;

impl Lexer {
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
    pub(super) fn lex_character_literal(&mut self) -> Result<Token, LexError> {
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
            kind: TokenKind::Literal(LiteralKind::CharacterLiteral(ch)),
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
    pub(super) fn lex_string_literal(&mut self) -> Result<Token, LexError> {
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
            kind: TokenKind::Literal(LiteralKind::StringLiteral(decoded)),
            span,
            lexeme,
        })
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
    pub(super) fn lex_identifier_or_keyword(&mut self) -> Result<Token, LexError> {
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
    /// Supports:
    /// - Signed integers: `123`
    /// - Unsigned integers: `123u`
    /// - Floating point numbers: `123.45`
    ///
    /// # Returns
    ///
    /// - `Ok(Token)` with `TokenKind::IntLiteral` for signed integers
    /// - `Ok(Token)` with `TokenKind::UnsignedIntLiteral` for unsigned integers (ending with `u`)
    /// - `Ok(Token)` with `TokenKind::FloatLiteral` for floating point numbers
    /// - `Err(LexError::InvalidNumber)` if the number is malformed or out of range
    /// - `Err(LexError::InvalidNumber)` if `u` suffix is used with a decimal point
    pub(super) fn lex_number(&mut self) -> Result<Token, LexError> {
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

        // Check for 'u' suffix (unsigned integer indicator)
        let is_unsigned = self.stream.peek() == Some(b'u');
        if is_unsigned {
            // Error: cannot use 'u' suffix with floating point numbers
            if is_float {
                let (end_idx, _, _) = self.stream.current_position();
                let lexeme_bytes = self.stream.slice(lex_start, end_idx);
                let lexeme = String::from_utf8_lossy(lexeme_bytes).to_string();
                return Err(LexError::InvalidNumber {
                    lexeme,
                    line: start_line,
                    column: start_col,
                });
            }
            self.stream.advance(); // consume 'u'
        }

        let (end_idx, end_line, end_col) = self.stream.current_position();

        // Get the lexeme as a string
        let lexeme_bytes = self.stream.slice(lex_start, end_idx);
        let lexeme = String::from_utf8_lossy(lexeme_bytes).to_string();

        // Parse as integer or float
        let kind = if is_float {
            // Validate the float by parsing it
            match lexeme.parse::<f64>() {
                Ok(f) => TokenKind::Literal(LiteralKind::FloatLiteral(f)),
                Err(_) => {
                    return Err(LexError::InvalidNumber {
                        lexeme,
                        line: start_line,
                        column: start_col,
                    });
                }
            }
        } else if is_unsigned {
            // Try to parse as unsigned integer (remove the 'u' suffix)
            let num_str = &lexeme[..lexeme.len() - 1];
            match num_str.parse::<u64>() {
                Ok(val) => TokenKind::Literal(LiteralKind::UnsignedIntLiteral(val)),
                Err(_) => {
                    return Err(LexError::InvalidNumber {
                        lexeme,
                        line: start_line,
                        column: start_col,
                    });
                }
            }
        } else {
            // Try to parse as signed integer
            match lexeme.parse::<i64>() {
                Ok(val) => TokenKind::Literal(LiteralKind::IntLiteral(val)),
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
}