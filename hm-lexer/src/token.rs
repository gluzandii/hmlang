pub mod span;
pub mod tokenkind;

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Token {
    pub kind: tokenkind::TokenKind,
    pub span: span::Span,
    pub lexeme: String,
}
