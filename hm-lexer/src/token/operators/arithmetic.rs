/// Arithmetic operators.
///
/// # Variants
/// /// - `Plus`: Addition operator (`+`)
/// - `Minus`: Subtraction operator (`-`)
/// - `Asterisk`: Multiplication operator (`*`)
/// - `Slash`: Division operator (`/`)
/// - `Percent`: Modulus operator (`%`)
/// - `Exponent`: Exponentiation operator (`**`)
#[cfg_attr(debug_assertions, derive(Debug, Clone, PartialEq, Eq))]
pub enum ArithmeticOperator {
    Plus,        // +
    Minus,       // -
    Asterisk,    // *
    Slash,       // /
    Modulo,     // %
    Exponent,    // **
}