/// Represents all reserved keywords in the language grammar.
///
/// This enum is used by the lexer and parser to classify tokens
/// that have special syntactic meaning.
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum Keywords {
    /// Keywords that affect control flow (branching, looping, returning)
    /// Declares a function
    Func,
    /// Returns a value from a function
    Return,
    /// Starts a conditional branch
    If,
    /// Alternative branch when condition is false
    Else,
    /// Additional conditional branch
    Elif,
    /// Starts a loop construct
    Loop,
    /// Starts a multi-branch selection
    Switch,
    /// Branch inside a switch statement
    Case,

    /// Keywords for variable and constant bindings
    /// Mutable variable declaration
    Var,
    /// Compile-time constant declaration
    Const,
    /// Immutable variable after initialization
    Final,

    /// Keywords for data types
    Type(TypeKind),

    /// Module Import
    Import,
}

/// Represents built-in data types in the language.
/// This enum is used to classify type keywords.
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum TypeKind {
    /// 8-bit signed integer
    Int8,
    /// 16-bit signed integer
    Int16,
    /// 32-bit signed integer
    Int32,
    /// 64-bit signed integer
    Int64,
    /// 8-bit unsigned integer
    Unsigned8,
    /// 16-bit unsigned integer
    Unsigned16,
    /// 32-bit unsigned integer
    Unsigned32,
    /// 64-bit unsigned integer
    Unsigned64,

    /// Floating point numeric types
    /// Single-precision floating point
    Float32,
    /// Double-precision floating point
    Float64,

    /// Other built-in or composite types
    /// UTF-8 encoded string type
    String,
    /// Single Unicode scalar value
    Char,
    /// User-defined composite data type
    Struct,

    /// Boolean type
    Bool,
}