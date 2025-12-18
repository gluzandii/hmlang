/// Assignment operators used in the language.
/// This enum represents various assignment operations.
///
/// # Variants
/// - `Assign`: Represents the simple assignment operator (`=`).
/// - `AddAssign`: Represents the addition assignment operator (`+=`).
/// - `SubtractAssign`: Represents the subtraction assignment operator (`-=`).
/// - `MultiplyAssign`: Represents the multiplication assignment operator (`*=`).
/// - `DivideAssign`: Represents the division assignment operator (`/=`).
/// - `ModuloAssign`: Represents the modulo assignment operator (`%=`).
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum AssignmentOperator {
    Assign,         // =
    AddAssign,      // +=
    SubtractAssign, // -=
    MultiplyAssign, // *=
    DivideAssign,   // /=
    ModuloAssign,   // %=
}