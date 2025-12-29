//! Operator type definitions for the Hummingbird language.
//!
//! This module contains all operator classifications used in token kinds:
//! - [`arithmetic`]: Arithmetic operators (`+`, `-`, `*`, `/`, `%`, `**`)
//! - [`relational`]: Comparison operators (`<`, `>`, `<=`, `>=`, `==`, `!=`)
//! - [`logical`]: Boolean operators (`&&`, `||`, `!`)
//! - [`assignment`]: Assignment operators (`=`, `+=`, `-=`, `*=`, `/=`, `%=`)
//! - [`bitwise`]: Bitwise operators (`&`, `|`, `^`, `~`, `<<`, `>>`)

pub mod arithmetic;
pub mod relational;
pub mod logical;
pub mod assignment;
pub mod bitwise;

/// Special operators not covered by other categories.
////
/// This enum includes operators like pointer access and scope resolution.
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum SpecialOps {
    /// Pointer access operator `->`
    PointerAccess,

    /// Scope resolution operator `::`
    ScopingOperator,
}