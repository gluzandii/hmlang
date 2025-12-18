/// Logical operators used in expressions.
///
/// # Variants
/// - `And`: Represents the logical AND operator (`&&`).
/// - `Or`: Represents the logical OR operator (`||`).
/// - `Not`: Represents the logical NOT operator (`!`).
#[cfg_attr(debug_assertions, derive(Debug, Clone, PartialEq, Eq))]
pub enum LogicalOperator {
    And, // &&
    Or,  // ||
    Not, // !
}