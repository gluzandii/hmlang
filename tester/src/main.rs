use std::str::FromStr;

use hm_lexer::charstream::CharStream;

fn main() {
    use hm_lexer::lexer::Lexer;
    let lexer =
        Lexer::new(CharStream::from_str("func main(): u32 { return 2320u; }").expect("Failed to create CharStream"));

    for token in lexer {
        let token = token.expect("Failed to get next token");
        println!("{:?}", token);
    }
}
