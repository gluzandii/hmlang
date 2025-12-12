#[cfg_attr(debug_assertions, derive(Debug))]
pub enum TokenKind {
    // Control Flow Keywords
    Func,
    Return,
    If,
    Else,
    Elif,
    Loop,
    Switch,
    Case,

    // Variable/Binding Keywords
    Var,
    Const,
    Final,

    // Integer Types
    Int8,
    Int16,
    Int32,
    Int64,
    Unsigned8,
    Unsigned16,
    Unsigned32,
    Unsigned64,

    // Floating Point Types
    Float,
    Double,

    // Other Types
    String,
    Character,
    Struct,

    // Identifiers and Literals
    Identifier(String),
    StringLiteral(String),
    CharacterLiteral(char),
    IntLiteral(i64),
    UnsignedIntLiteral(u64),
    FloatLiteral(String),

    // Delimiters
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]

    // Operators and Punctuation
    Colon,     // :
    Semicolon, // ;
    Comma,     // ,
    Dot,       // .

    // Special
    Eof,
}

impl TokenKind {
    pub fn keyword(s: &str) -> Option<Self> {
        match s {
            // Control Flow
            "func" => Some(TokenKind::Func),
            "return" => Some(TokenKind::Return),
            "if" => Some(TokenKind::If),
            "else" => Some(TokenKind::Else),
            "elif" => Some(TokenKind::Elif),
            "loop" => Some(TokenKind::Loop),
            "switch" => Some(TokenKind::Switch),
            "case" => Some(TokenKind::Case),

            // Variable/Binding
            "var" => Some(TokenKind::Var),
            "const" => Some(TokenKind::Const),
            "final" => Some(TokenKind::Final),

            // Integer Types
            "int8" => Some(TokenKind::Int8),
            "int16" => Some(TokenKind::Int16),
            "int32" => Some(TokenKind::Int32),
            "int64" => Some(TokenKind::Int64),
            "unsigned8" => Some(TokenKind::Unsigned8),
            "unsigned16" => Some(TokenKind::Unsigned16),
            "unsigned32" => Some(TokenKind::Unsigned32),
            "unsigned64" => Some(TokenKind::Unsigned64),

            // Floating Point Types
            "float" => Some(TokenKind::Float),
            "double" => Some(TokenKind::Double),

            // Other Types
            "string" => Some(TokenKind::String),
            "character" => Some(TokenKind::Character),
            "struct" => Some(TokenKind::Struct),

            _ => None,
        }
    }
}
