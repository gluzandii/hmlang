use crate::charstream::CharStream;
use crate::lexerror::LexError;
use crate::token::Token;

pub struct Lexer {
    stream: CharStream,
}

impl Lexer {
    pub fn new(stream: CharStream) -> Self {
        Self { stream }
    }

    pub fn next_token(&mut self) -> Result<Token, LexError> {
        todo!()
    }
}
