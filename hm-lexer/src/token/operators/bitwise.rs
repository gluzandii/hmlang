/// Bitwise operators used in expressions.
///
/// # Variants
/// - `And`: Represents the bitwise AND operator (`&`).
/// - `Or`: Represents the bitwise OR operator (`|`).
/// - `Xor`: Represents the bitwise XOR operator (`^`).
/// - `Not`: Represents the bitwise NOT operator (`~`).
/// - `LeftShift`: Represents the left shift operator (`<<`).
/// - `RightShift`: Represents the right shift operator (`>>`).
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum BitwiseOperator {
    And,        // &
    Or,         // |
    Xor,        // ^
    Not,        // ~
    LeftShift,  // <<
    RightShift, // >>
}