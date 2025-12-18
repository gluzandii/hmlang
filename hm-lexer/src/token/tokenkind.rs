//! Token type classifications for the Hummingbird language.
//!
//! `TokenKind` enumerates all possible token types the lexer can produce,
//! including keywords, identifiers, literals, delimiters, and operators.

use crate::token::operators::arithmetic::ArithmeticOperator;
use crate::token::operators::logical::LogicalOperator;
use crate::token::operators::relational::RelationalOperator;


/// The type and classification of a token produced by the lexer.
///
/// `TokenKind` represents the semantic category of a token, distinguishing
/// between keywords (reserved words), identifiers (variable names), various
/// types of literals, and punctuation/operators.
///
/// # Variants
///
/// ## Keywords
/// - Control flow: `Func`, `Return`, `If`, `Else`, `Elif`, `Loop`, `Switch`, `Case`
/// - Declarations: `Var`, `Const`, `Final`
/// - Types: `Int8`, `Int16`, `Int32`, `Int64`, `Unsigned8`, `Unsigned16`, `Unsigned32`, `Unsigned64`, `Float`, `Double`, `String`, `Character`, `Struct`
///
/// ## Identifiers and Literals
/// - `Identifier(String)`: User-defined names
/// - `StringLiteral(String)`: Double-quoted strings
/// - `CharacterLiteral(char)`: Single-quoted characters
/// - `IntLiteral(i64)`: Signed integer constants
/// - `UnsignedIntLiteral(u64)`: Unsigned integer constants
/// - `FloatLiteral(String)`: Floating-point constants
///
/// ## Delimiters
/// - Parentheses: `LeftParen`, `RightParen`
/// - Braces: `LeftBrace`, `RightBrace`
/// - Brackets: `LeftBracket`, `RightBracket`
///
/// ## Operators and Punctuation
/// - `Colon`, `Semicolon`, `Comma`, `Dot`
///
/// ## Special
/// - `Eof`: End of file marker
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
    Void,

    // Identifiers and Literals
    Identifier(String),
    StringLiteral(String),
    CharacterLiteral(char),
    IntLiteral(i64),
    UnsignedIntLiteral(u64),
    FloatLiteral(f64),

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

    // Arithmetic Operators
    // +, -, *, /, %, **
    ArithmeticOperator(ArithmeticOperator),

    // Relational Operators
    // <, <=, >, >=, ==, !=
    RelationalOperator(RelationalOperator),

    // Logical Operators
    // &&, ||, !
    LogicalOperator(LogicalOperator),

    Equal,       // =
    Ampersand,   // &
    Pipe,        // |
    Caret,       // ^
    Tilde,       // ~
    Question,    // ?
    Modulo,      // %

    // Special
    Eof,
}

impl TokenKind {
    /// Attempt to parse a string as a reserved keyword.
    ///
    /// This method checks if the provided string exactly matches a reserved
    /// keyword and returns the corresponding token kind if found. It is
    /// case-sensitive and matches the exact keyword names.
    ///
    /// # Arguments
    ///
    /// * `s` - The string to check as a potential keyword
    ///
    /// # Returns
    ///
    /// - `Some(TokenKind)` if `s` is a recognized keyword
    /// - `None` if `s` is not a reserved keyword (user-defined identifier)
    ///
    /// # Example
    ///
    /// ```
    /// # use hm_lexer::token::tokenkind::TokenKind;
    /// assert_eq!(TokenKind::keyword("if"), Some(TokenKind::If));
    /// assert_eq!(TokenKind::keyword("myVar"), None);
    /// assert_eq!(TokenKind::keyword("int32"), Some(TokenKind::Int32));
    /// ```
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
            "void" => Some(TokenKind::Void),

            _ => None,
        }
    }
}
