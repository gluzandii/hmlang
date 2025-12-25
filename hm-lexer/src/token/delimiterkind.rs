//! Delimiter and punctuation token types for the Hummingbird language.

/// Represents all delimiter and punctuation tokens.
///
/// Used to group expressions, separate statements, and mark boundaries in code.
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum DelimiterKind {
    /// Left parenthesis `(`
    LeftParen,
    /// Right parenthesis `)`
    RightParen,
    /// Left brace `{`
    LeftBrace,
    /// Right brace `}`
    RightBrace,
    /// Left bracket `[`
    LeftBracket,
    /// Right bracket `]`
    RightBracket,
    /// Colon `:`
    Colon,
    /// Semicolon `;`
    Semicolon,
    /// Comma `,`
    Comma,
    /// Dot `.`
    Dot,

    /// Question mark `?`
    QuestionMark,
}