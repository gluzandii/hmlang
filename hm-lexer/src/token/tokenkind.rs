//! Token type classifications for the Hummingbird language.
//!
//! `TokenKind` enumerates all possible token types the lexer can produce,
//! including keywords, identifiers, literals, delimiters, and operators.

use crate::token::operators::arithmetic::ArithmeticOperator;
use crate::token::operators::assignment::AssignmentOperator;
use crate::token::operators::bitwise::BitwiseOperator;
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
    /// Function declaration keyword (`func`)
    Func,
    /// Return statement keyword (`return`)
    Return,
    /// If conditional keyword (`if`)
    If,
    /// Else conditional keyword (`else`)
    Else,
    /// Else-if conditional keyword (`elif`)
    Elif,
    /// Loop keyword (`loop`)
    Loop,
    /// Switch statement keyword (`switch`)
    Switch,
    /// Case statement keyword (`case`)
    Case,

    // Variable/Binding Keywords
    /// Variable declaration keyword (`var`)
    Var,
    /// Constant declaration keyword (`const`)
    Const,
    /// Final declaration keyword (`final`)
    Final,

    // Integer Types
    /// 8-bit signed integer type (`int8`)
    Int8,
    /// 16-bit signed integer type (`int16`)
    Int16,
    /// 32-bit signed integer type (`int32`)
    Int32,
    /// 64-bit signed integer type (`int64`)
    Int64,
    /// 8-bit unsigned integer type (`unsigned8`)
    Unsigned8,
    /// 16-bit unsigned integer type (`unsigned16`)
    Unsigned16,
    /// 32-bit unsigned integer type (`unsigned32`)
    Unsigned32,
    /// 64-bit unsigned integer type (`unsigned64`)
    Unsigned64,

    // Floating Point Types
    /// Single-precision floating point type (`float`)
    Float,
    /// Double-precision floating point type (`double`)
    Double,

    // Other Types
    /// String type keyword (`string`)
    String,
    /// Character type keyword (`character`)
    Character,
    /// Structure type keyword (`struct`)
    Struct,
    /// Void type keyword (`void`)
    Void,

    // Identifiers and Literals
    /// User-defined identifier (variable, function name, etc.)
    Identifier(String),
    /// String literal value (e.g., `"hello"`)
    StringLiteral(String),
    /// Character literal value (e.g., `'a'`)
    CharacterLiteral(char),
    /// Signed integer literal value
    IntLiteral(i64),
    /// Unsigned integer literal value
    UnsignedIntLiteral(u64),
    /// Floating point literal value
    FloatLiteral(f64),

    // Delimiters
    /// Left parenthesis `(`
    LeftParen,
    /// Right parenthesis `)`
    RightParen,
    /// Left brace `{`
    LeftBrace,
    /// Right brace `}`
    RightBrace,
    /// Left bracket `[`
    LeftBracket,
    /// Right bracket `]`
    RightBracket,

    // Operators and Punctuation
    /// Colon `:`
    Colon,
    /// Semicolon `;`
    Semicolon,
    /// Comma `,`
    Comma,
    /// Dot `.`
    Dot,

    /// Scope resolution operator `::`
    ScopingOperator,

    // Arithmetic Operators
    /// Arithmetic operator (`+`, `-`, `*`, `/`, `%`, `**`)
    ArithmeticOperator(ArithmeticOperator),

    // Relational Operators
    /// Relational/comparison operator (`<`, `<=`, `>`, `>=`, `==`, `!=`)
    RelationalOperator(RelationalOperator),

    // Logical Operators
    /// Logical operator (`&&`, `||`, `!`)
    LogicalOperator(LogicalOperator),

    // Assignment Operators
    /// Assignment operator (`=`, `+=`, `-=`, `*=`, `/=`, `%=`)
    AssignmentOperator(AssignmentOperator),

    // Bitwise Operators
    /// Bitwise operator (`&`, `|`, `^`, `~`, `<<`, `>>`)
    BitwiseOperator(BitwiseOperator),

    /// Pointer access operator `->`
    PointerAccessOperator,

    /// Question mark `?`
    QuestionMark,

    // Special
    /// End of file marker
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
    /// // Returns Some(TokenKind) for keywords
    /// assert!(TokenKind::keyword("if").is_some());
    /// assert!(TokenKind::keyword("int32").is_some());
    /// // Returns None for non-keywords
    /// assert!(TokenKind::keyword("myVar").is_none());
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
