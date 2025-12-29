//! Token type classifications for the Hummingbird language.
//!
//! `TokenKind` enumerates all possible token types the lexer can produce,
//! including keywords, identifiers, literals, delimiters, and operators.

use crate::token::delimiters::Delimiters;
use crate::token::keywords::Keywords;
use crate::token::keywords::TypeKind;
use crate::token::literals::Literals;
use crate::token::operators::arithmetic::ArithmeticOps;
use crate::token::operators::assignment::AssignmentOps;
use crate::token::operators::bitwise::BitwiseOps;
use crate::token::operators::logical::LogicalOps;
use crate::token::operators::relational::RelationalOps;
use crate::token::operators::SpecialOps;

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
    // Keywords
    /// Reserved keyword in the HM language
    Keyword(Keywords),

    // Identifiers and Literals
    /// User-defined identifier (variable, function name, etc.)
    Identifier(String),

    /// All literal types
    Literal(Literals),

    /// Delimiter symbols (parentheses, braces, brackets, etc.)
    Delimiter(Delimiters),

    // Arithmetic Operators
    /// Arithmetic operator (`+`, `-`, `*`, `/`, `%`, `**`)
    ArithmeticOperator(ArithmeticOps),

    // Relational Operators
    /// Relational/comparison operator (`<`, `<=`, `>`, `>=`, `==`, `!=`)
    RelationalOperator(RelationalOps),

    // Logical Operators
    /// Logical operator (`&&`, `||`, `!`)
    LogicalOperator(LogicalOps),

    // Assignment Operators
    /// Assignment operator (`=`, `+=`, `-=`, `*=`, `/=`, `%=`)
    AssignmentOperator(AssignmentOps),

    // Bitwise Operators
    /// Bitwise operator (`&`, `|`, `^`, `~`, `<<`, `>>`)
    BitwiseOperator(BitwiseOps),

    /// Special operators (`::`, `->`)
    SpecialOperator(SpecialOps),

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
        let kw = match s {
            // Control Flow
            "func" => Some(Keywords::Func),
            "return" => Some(Keywords::Return),
            "if" => Some(Keywords::If),
            "else" => Some(Keywords::Else),
            "elif" => Some(Keywords::Elif),
            "loop" => Some(Keywords::Loop),
            "switch" => Some(Keywords::Switch),
            "case" => Some(Keywords::Case),

            // Variable/Binding
            "var" => Some(Keywords::Var),
            "const" => Some(Keywords::Const),
            "final" => Some(Keywords::Final),

            // Integer Types
            "i8" => Some(Keywords::Type(TypeKind::Int8)),
            "i16" => Some(Keywords::Type(TypeKind::Int16)),
            "i32" => Some(Keywords::Type(TypeKind::Int32)),
            "i64" => Some(Keywords::Type(TypeKind::Int64)),
            "u8" => Some(Keywords::Type(TypeKind::Unsigned8)),
            "u16" => Some(Keywords::Type(TypeKind::Unsigned16)),
            "u32" => Some(Keywords::Type(TypeKind::Unsigned32)),
            "u64" => Some(Keywords::Type(TypeKind::Unsigned64)),

            // Floating Point Types
            "f32" => Some(Keywords::Type(TypeKind::Float32)),
            "f64" => Some(Keywords::Type(TypeKind::Float64)),

            // Other Types
            "string" => Some(Keywords::Type(TypeKind::String)),
            "char" => Some(Keywords::Type(TypeKind::Char)),
            "struct" => Some(Keywords::Type(TypeKind::Struct)),
            "bool" => Some(Keywords::Type(TypeKind::Bool)),

            "import" => Some(Keywords::Import),

            _ => None,
        };

        kw.map(TokenKind::Keyword)
    }
}
