//! Assignment operator types.

/// Assignment operators for variable assignment and compound assignments.
///
/// These operators assign values to variables. Compound assignment operators
/// combine an arithmetic operation with assignment (e.g., `a += b` is equivalent to `a = a + b`).
///
/// # Variants
///
/// - `Assign`: Simple assignment (`=`)
/// - `AddAssign`: Addition assignment (`+=`)
/// - `SubtractAssign`: Subtraction assignment (`-=`)
/// - `MultiplyAssign`: Multiplication assignment (`*=`)
/// - `DivideAssign`: Division assignment (`/=`)
/// - `ModuloAssign`: Modulo assignment (`%=`)
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum AssignmentOperator {
    /// Simple assignment operator (`=`)
    Assign,
    /// Addition assignment operator (`+=`)
    AddAssign,
    /// Subtraction assignment operator (`-=`)
    SubtractAssign,
    /// Multiplication assignment operator (`*=`)
    MultiplyAssign,
    /// Division assignment operator (`/=`)
    DivideAssign,
    /// Modulo assignment operator (`%=`)
    ModuloAssign,
}