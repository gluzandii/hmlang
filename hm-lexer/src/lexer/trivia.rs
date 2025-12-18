use crate::lexer::Lexer;

impl Lexer {
    /// Skip whitespace and comments until meaningful content is found.
    ///
    /// Trivia includes:
    /// - Whitespace: spaces, tabs, carriage returns, newlines
    /// - Line comments: `// ...` until end of line
    /// - Block comments: `/* ... */` with nesting support
    ///
    /// The stream position advances past all trivia, leaving the cursor
    /// at either a non-trivia character or EOF.
    pub(super) fn skip_trivia(&mut self) {
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
}