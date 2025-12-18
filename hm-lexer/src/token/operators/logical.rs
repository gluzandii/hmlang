//! Logical (boolean) operator types.

/// Logical operators for boolean operations.
///
/// These operators perform logical operations on boolean values.
/// `And` and `Or` are binary operators, while `Not` is a unary operator.
///
/// # Variants
///
/// - `And`: Logical AND operator (`&&`) - true if both operands are true
/// - `Or`: Logical OR operator (`||`) - true if at least one operand is true
/// - `Not`: Logical NOT operator (`!`) - inverts a boolean value
#[cfg_attr(debug_assertions, derive(Debug, Clone, PartialEq, Eq))]
pub enum LogicalOperator {
    /// Logical AND operator (`&&`)
    And,
    /// Logical OR operator (`||`)
    Or,
    /// Logical NOT operator (`!`)
    Not,
}