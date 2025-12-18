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