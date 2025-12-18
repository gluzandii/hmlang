/// Macro to decode escape sequences in string literals.
/// It consumes the backslash and checks the next character to determine
/// the appropriate escape sequence. If the escape sequence is valid, it
/// returns the corresponding character. If invalid, it returns a LexError.
#[macro_export]
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

/// Macro to create a single-character token.
/// It advances the lexer stream, captures the current position,
/// and constructs a Token with the provided kind and lexeme.
#[macro_export]
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
