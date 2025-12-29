//! Arithmetic operator types for mathematical operations.

/// Arithmetic operators for mathematical operations.
///
/// These operators perform basic arithmetic operations on numeric values.
/// All operators are binary except when used in specific contexts (e.g., unary minus).
///
/// # Variants
///
/// - `Plus`: Addition operator (`+`)
/// - `Minus`: Subtraction operator (`-`)
/// - `Asterisk`: Multiplication operator (`*`)
/// - `Slash`: Division operator (`/`)
/// - `Modulo`: Modulus/remainder operator (`%`)
/// - `Exponent`: Exponentiation operator (`**`)
#[cfg_attr(debug_assertions, derive(Debug, Clone, PartialEq, Eq))]
pub enum ArithmeticOps {
    /// Addition operator (`+`)
    Plus,
    /// Subtraction operator (`-`)
    Minus,
    /// Multiplication operator (`*`)
    Asterisk,
    /// Division operator (`/`)
    Slash,
    /// Modulus/remainder operator (`%`)
    Modulo,
    /// Exponentiation operator (`**`)
    Exponent,
}