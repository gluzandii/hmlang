/// Relational operators used in expressions.
///
/// # Variants
/// - `LessThan`: Represents the `<` operator.
/// - `GreaterThan`: Represents the `>` operator.
/// - `LessThanOrEqual`: Represents the `<=` operator.
/// - `GreaterThanOrEqual`: Represents the `>=` operator.
/// - `Equal`: Represents the `==` operator.
/// - `NotEqual`: Represents the `!=` operator.
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum RelationalOperator {
    LessThan,          // <
    GreaterThan,       // >
    LessThanOrEqual,   // <=
    GreaterThanOrEqual,// >=
    Equal,             // ==
    NotEqual,          // !=
}