use std::str::FromStr;

use hm_lexer::charstream::CharStream;

fn main() {
    use hm_lexer::lexer::Lexer;
    let mut lexer =
        Lexer::new(CharStream::from_str("var x = 10;").expect("Failed to create CharStream"));

    loop {
        let token = lexer.next_token().expect("Failed to get next token");
        if token.is_eof() {
            break;
        }
        println!("{:?}", token);
    }
}
