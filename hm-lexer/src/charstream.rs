use std::str::FromStr;

use crate::lexerror::LexError;

/// An ASCII-only cursor over an in-memory byte buffer for lexer frontends.
///
/// `CharStream` intentionally works on raw bytes (`u8`) instead of `char` so a
/// lexer can operate without assuming UTF-8 validity. The stream owns its
/// buffer, keeps a current index, and tracks human-friendly 1-based line and
/// column numbers as bytes are consumed. It never reads from an external
/// source and never advances past `input.len()`, making it deterministic and
/// replayable for tokenization.
pub struct CharStream {
    /// Owned input buffer containing the raw bytes to be consumed by the lexer.
    /// No UTF-8 assumptions are made; bytes are treated as ASCII code units.
    input: Vec<u8>,

    /// Current byte index (0-based) into the input buffer.
    /// This always points to the next byte to be read.
    index: usize,

    /// Current line number (1-based) corresponding to the cursor position.
    /// Incremented whenever a newline (`b'\n'`) is consumed.
    line: usize,

    /// Current column number (1-based) corresponding to the cursor position.
    /// Reset to 1 after a newline, incremented for any other consumed byte.
    column: usize,
}

impl CharStream {
    /// Create a new stream from an existing byte buffer.
    ///
    /// The cursor starts at index 0 with line 1 and column 1. No UTF-8 decoding
    /// occurs; bytes are treated as ASCII code units.
    pub fn new(input: Vec<u8>) -> Result<Self, LexError> {
        if input.is_empty() {
            return Err(LexError::EmptyInput);
        }
        Ok(Self {
            input,
            index: 0,
            line: 1,
            column: 1,
        })
    }

    /// Create a stream by copying a byte slice into an owned buffer.
    #[inline]
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, LexError> {
        Self::new(bytes.to_vec())
    }

    /// Returns true when the cursor is at or beyond the end of the buffer.
    pub fn is_eof(&self) -> bool {
        self.index >= self.input.len()
    }

    /// Current byte offset (0-based) into the buffer.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Current 1-based line number for the cursor position.
    pub fn line(&self) -> usize {
        self.line
    }

    /// Current 1-based column number for the cursor position.
    pub fn column(&self) -> usize {
        self.column
    }

    /// Current (line, column) tuple.
    pub fn line_column(&self) -> (usize, usize) {
        (self.line, self.column)
    }

    /// Peek at the current byte without advancing.
    ///
    /// Returns `None` at end of input. Does not alter the cursor state.
    pub fn peek(&self) -> Option<u8> {
        self.peek_n(0)
    }

    /// Look ahead `n` bytes from the current position without advancing.
    ///
    /// `n = 0` is equivalent to [`peek`]. Returns `None` when the requested
    /// offset is past the end of the buffer.
    pub fn peek_n(&self, n: usize) -> Option<u8> {
        let idx = self.index.checked_add(n)?;
        self.input.get(idx).copied()
    }

    /// Consume and return the current byte, updating line and column counters.
    ///
    /// Newlines (`b'\n'`) increment the line and reset the column to 1. Any
    /// other byte increments the column. Returns `None` if already at EOF.
    pub fn advance(&mut self) -> Option<u8> {
        if self.is_eof() {
            return None;
        }

        let b = self.input[self.index];
        self.index += 1;

        if b == b'\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        Some(b)
    }

    /// Advance if the next byte matches `expected`.
    ///
    /// Returns `true` when a match occurs and consumes the byte, `false`
    /// otherwise.
    pub fn match_byte(&mut self, expected: u8) -> bool {
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    /// Borrow a slice of the underlying buffer for lexeme extraction.
    ///
    /// Panics if the range is out of bounds, matching normal slice behavior.
    pub fn slice(&self, start: usize, end: usize) -> &[u8] {
        &self.input[start..end]
    }

    /// Consume bytes while a predicate holds, returning the consumed span.
    ///
    /// The returned `(start, end)` indices use the stream's byte offsets and
    /// can be fed back into [`slice`] for lexeme capture.
    pub fn consume_while<F: Fn(u8) -> bool>(&mut self, f: F) -> (usize, usize) {
        let start = self.index;
        while let Some(b) = self.peek() {
            if f(b) {
                self.advance();
            } else {
                break;
            }
        }
        let end = self.index;
        (start, end)
    }

    /// Skip bytes while a predicate holds, discarding the consumed span.
    pub fn skip_while<F: Fn(u8) -> bool>(&mut self, f: F) {
        while let Some(b) = self.peek() {
            if f(b) {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Skip ASCII whitespace (space, tab, carriage return, newline).
    pub fn skip_whitespace(&mut self) {
        self.skip_while(|b| matches!(b, b' ' | b'\t' | b'\r' | b'\n'));
    }

    /// Snapshot the current byte index and line/column for token starts.
    pub fn current_start_pos(&self) -> (usize, usize, usize) {
        (self.index, self.line, self.column)
    }
}

impl FromStr for CharStream {
    type Err = LexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_bytes(s.as_bytes())
    }
}
