//! Literal token types for the Hummingbird language.
//!
//! `Literals` enumerates all possible literal values that can appear in source code,
//! including strings, characters, integers, and floating-point numbers.

/// Represents all literal value types recognized by the lexer.
///
/// A literal is a fixed value written directly in the source code. This enum
/// captures the different categories of literals with their parsed values.
///
/// # Variants
///
/// - `StringLiteral(String)`: A double-quoted string literal
/// - `CharacterLiteral(char)`: A single-quoted character literal
/// - `IntLiteral(i64)`: A signed integer literal
/// - `UnsignedIntLiteral(u64)`: An unsigned integer literal
/// - `FloatLiteral(f64)`: A floating-point literal
///
/// # Example
///
/// ```
/// # use hm_lexer::token::literals::Literals;
/// let str_lit = Literals::StringLiteral("hello".to_string());
/// let int_lit = Literals::IntLiteral(42);
/// let float_lit = Literals::FloatLiteral(3.14);
/// ```
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Literals {
    /// String literal value (e.g., `"hello"`)
    StringLiteral(String),
    /// Character literal value (e.g., `'a'`)
    CharacterLiteral(char),
    /// Signed integer literal value
    IntLiteral(i64),
    /// Unsigned integer literal value
    UnsignedIntLiteral(u64),
    /// Floating point literal value (e.g., `3.14`, `0.5`, `-2.0`)
    FloatLiteral(f64),
}