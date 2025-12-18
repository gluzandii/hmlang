//! Source location tracking types.

/// A contiguous region (span) of source text represented as byte offsets
/// and human-readable line/column positions.
///
/// `Span` stores both byte offsets into the source (`start` and `end`) and
/// the corresponding 1-based line/column coordinates so callers can display
/// or compute human-friendly locations without re-scanning the source.
///
/// # Invariants
///
/// - `start <= end` and `end` is exclusive (one past the last byte in the span)
/// - Line and column numbers are 1-based (first line is 1, first column is 1)
///
/// # Example
///
/// ```no_run
/// # use hm_lexer::token::span::Span;
/// let span = Span {
///     start: 0,
///     end: 5,
///     line_start: 1,
///     column_start: 1,
///     line_end: 1,
///     column_end: 6,
/// };
/// // Represents "hello" at line 1, columns 1-5
/// ```
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Span {
    /// Byte offset of the first byte in the span (inclusive).
    pub start: usize,

    /// Byte offset one past the last byte in the span (exclusive).
    pub end: usize,

    /// 1-based line number where the span starts.
    pub line_start: usize,

    /// 1-based column number within `line_start` where the span starts.
    pub column_start: usize,

    /// 1-based line number where the span ends.
    pub line_end: usize,

    /// 1-based column number within `line_end` where the span ends.
    pub column_end: usize,
}
