//! Relational (comparison) operator types.

/// Relational operators used for comparing values.
///
/// These operators compare two values and produce a boolean result.
/// They are commonly used in conditional expressions and control flow statements.
///
/// # Variants
///
/// - `LessThan`: Less than comparison (`<`)
/// - `GreaterThan`: Greater than comparison (`>`)
/// - `LessThanOrEqual`: Less than or equal comparison (`<=`)
/// - `GreaterThanOrEqual`: Greater than or equal comparison (`>=`)
/// - `Equal`: Equality comparison (`==`)
/// - `NotEqual`: Inequality comparison (`!=`)
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum RelationalOperator {
    /// Less than operator (`<`)
    LessThan,
    /// Greater than operator (`>`)
    GreaterThan,
    /// Less than or equal operator (`<=`)
    LessThanOrEqual,
    /// Greater than or equal operator (`>=`)
    GreaterThanOrEqual,
    /// Equality operator (`==`)
    Equal,
    /// Inequality operator (`!=`)
    NotEqual,
}