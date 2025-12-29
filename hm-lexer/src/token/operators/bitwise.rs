//! Bitwise operator types for bit manipulation.

/// Bitwise operators for bit-level operations on integer values.
///
/// These operators perform operations on the individual bits of integer values.
/// They are useful for low-level programming, flags, and optimization.
///
/// # Variants
///
/// - `And`: Bitwise AND (`&`) - sets each bit to 1 if both bits are 1
/// - `Or`: Bitwise OR (`|`) - sets each bit to 1 if at least one bit is 1
/// - `Xor`: Bitwise XOR (`^`) - sets each bit to 1 if only one bit is 1
/// - `Not`: Bitwise NOT (`~`) - inverts all bits
/// - `LeftShift`: Left shift (`<<`) - shifts bits left, filling with zeros
/// - `RightShift`: Right shift (`>>`) - shifts bits right
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum BitwiseOps {
    /// Bitwise AND operator (`&`)
    And,
    /// Bitwise OR operator (`|`)
    Or,
    /// Bitwise XOR operator (`^`)
    Xor,
    /// Bitwise NOT operator (`~`)
    Not,
    /// Left shift operator (`<<`)
    LeftShift,
    /// Right shift operator (`>>`)
    RightShift,
}