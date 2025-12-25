//! Token type classifications for the Hummingbird language.
//!
//! `TokenKind` enumerates all possible token types the lexer can produce,
//! including keywords, identifiers, literals, delimiters, and operators.

use crate::token::delimiterkind::DelimiterKind;
use crate::token::keywordkind::KeywordKind;
use crate::token::keywordkind::TypeKind;
use crate::token::literalkind::LiteralKind;
use crate::token::operators::arithmetic::ArithmeticOperator;
use crate::token::operators::assignment::AssignmentOperator;
use crate::token::operators::bitwise::BitwiseOperator;
use crate::token::operators::logical::LogicalOperator;
use crate::token::operators::relational::RelationalOperator;
use crate::token::operators::SpecialOperator;

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
    Keyword(KeywordKind),

    // Identifiers and Literals
    /// User-defined identifier (variable, function name, etc.)
    Identifier(String),

    /// All literal types
    Literal(LiteralKind),

    /// Delimiter symbols (parentheses, braces, brackets, etc.)
    Delimiter(DelimiterKind),

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

    /// Special operators (`::`, `->`)
    SpecialOperator(SpecialOperator),

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
            "func" => Some(KeywordKind::Func),
            "return" => Some(KeywordKind::Return),
            "if" => Some(KeywordKind::If),
            "else" => Some(KeywordKind::Else),
            "elif" => Some(KeywordKind::Elif),
            "loop" => Some(KeywordKind::Loop),
            "switch" => Some(KeywordKind::Switch),
            "case" => Some(KeywordKind::Case),

            // Variable/Binding
            "var" => Some(KeywordKind::Var),
            "const" => Some(KeywordKind::Const),
            "final" => Some(KeywordKind::Final),

            // Integer Types
            "i8" => Some(KeywordKind::Type(TypeKind::Int8)),
            "i16" => Some(KeywordKind::Type(TypeKind::Int16)),
            "i32" => Some(KeywordKind::Type(TypeKind::Int32)),
            "i64" => Some(KeywordKind::Type(TypeKind::Int64)),
            "u8" => Some(KeywordKind::Type(TypeKind::Unsigned8)),
            "u16" => Some(KeywordKind::Type(TypeKind::Unsigned16)),
            "u32" => Some(KeywordKind::Type(TypeKind::Unsigned32)),
            "u64" => Some(KeywordKind::Type(TypeKind::Unsigned64)),

            // Floating Point Types
            "f32" => Some(KeywordKind::Type(TypeKind::Float32)),
            "f64" => Some(KeywordKind::Type(TypeKind::Float64)),

            // Other Types
            "string" => Some(KeywordKind::Type(TypeKind::String)),
            "char" => Some(KeywordKind::Type(TypeKind::Char)),
            "struct" => Some(KeywordKind::Type(TypeKind::Struct)),
            "bool" => Some(KeywordKind::Type(TypeKind::Bool)),

            "import" => Some(KeywordKind::Import),

            _ => None,
        };

        kw.map(TokenKind::Keyword)
    }
}
