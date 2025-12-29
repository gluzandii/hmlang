//! Operator tokenization logic.
//!
//! This module handles tokenizing all operators including arithmetic,
//! assignment, relational, logical, and bitwise operators. It consolidates
//! the repetitive operator-matching logic from the main lexer.

use crate::charstream::CharStream;
use crate::lexerror::LexError;
use crate::token::operators::arithmetic::ArithmeticOps;
use crate::token::operators::assignment::AssignmentOps;
use crate::token::operators::bitwise::BitwiseOps;
use crate::token::operators::logical::LogicalOps;
use crate::token::operators::relational::RelationalOps;
use crate::token::operators::SpecialOps;
use crate::token::{tokenkind::TokenKind, Token};

use super::token_builder::TokenBuilder;

/// Tokenize an operator based on the starting byte.
///
/// This function delegates to specific operator lexing functions based
/// on the byte character. It handles both single-character operators
/// and multi-character operators (e.g., `==`, `+=`, `<<`).
///
/// # Arguments
///
/// * `stream` - The character stream
/// * `byte` - The starting byte of the operator
///
/// # Returns
///
/// The tokenized operator or an error
pub fn lex_operator(stream: &mut CharStream, byte: u8) -> Result<Token, LexError> {
    match byte {
        b'=' => lex_equals(stream),
        b'+' => lex_plus(stream),
        b'-' => lex_minus(stream),
        b'*' => lex_asterisk(stream),
        b'/' => lex_slash(stream),
        b'%' => lex_modulo(stream),
        b'<' => lex_less_than(stream),
        b'>' => lex_greater_than(stream),
        b'!' => lex_not(stream),
        b'&' => lex_ampersand(stream),
        b'|' => lex_pipe(stream),
        b'^' => {
            let builder = TokenBuilder::new(stream);
            Ok(builder.single_char_token(
                TokenKind::BitwiseOperator(BitwiseOps::Xor),
                "^",
            ))
        }
        b'~' => {
            let builder = TokenBuilder::new(stream);
            Ok(builder.single_char_token(
                TokenKind::BitwiseOperator(BitwiseOps::Not),
                "~",
            ))
        }
        _ => unreachable!("lex_operator called with non-operator byte: {}", byte as char),
    }
}

/// Tokenize `=` or `==`
fn lex_equals(stream: &mut CharStream) -> Result<Token, LexError> {
    let is_equal = stream.peek_n(1) == Some(b'=');
    let builder = TokenBuilder::new(stream);
    if is_equal {
        Ok(builder.multi_char_token(
            2,
            TokenKind::RelationalOperator(RelationalOps::Equal),
            "==",
        ))
    } else {
        Ok(builder.single_char_token(
            TokenKind::AssignmentOperator(AssignmentOps::Assign),
            "=",
        ))
    }
}

/// Tokenize `+` or `+=`
fn lex_plus(stream: &mut CharStream) -> Result<Token, LexError> {
    let is_assign = stream.peek_n(1) == Some(b'=');
    let builder = TokenBuilder::new(stream);
    if is_assign {
        Ok(builder.multi_char_token(
            2,
            TokenKind::AssignmentOperator(AssignmentOps::AddAssign),
            "+=",
        ))
    } else {
        Ok(builder.single_char_token(
            TokenKind::ArithmeticOperator(ArithmeticOps::Plus),
            "+",
        ))
    }
}

/// Tokenize `-` or `-=` or `->`
fn lex_minus(stream: &mut CharStream) -> Result<Token, LexError> {
    let next = stream.peek_n(1);
    let builder = TokenBuilder::new(stream);
    if next == Some(b'=') {
        Ok(builder.multi_char_token(
            2,
            TokenKind::AssignmentOperator(AssignmentOps::SubtractAssign),
            "-=",
        ))
    } else if next == Some(b'>') {
        Ok(builder.multi_char_token(
            2,
            TokenKind::SpecialOperator(SpecialOps::PointerAccess),
            "->",
        ))
    } else {
        Ok(builder.single_char_token(
            TokenKind::ArithmeticOperator(ArithmeticOps::Minus),
            "-",
        ))
    }
}

/// Tokenize `*`, `**`, or `*=`
fn lex_asterisk(stream: &mut CharStream) -> Result<Token, LexError> {
    let next = stream.peek_n(1);
    let builder = TokenBuilder::new(stream);
    if next == Some(b'=') {
        Ok(builder.multi_char_token(
            2,
            TokenKind::AssignmentOperator(AssignmentOps::MultiplyAssign),
            "*=",
        ))
    } else if next == Some(b'*') {
        Ok(builder.multi_char_token(
            2,
            TokenKind::ArithmeticOperator(ArithmeticOps::Exponent),
            "**",
        ))
    } else {
        Ok(builder.single_char_token(
            TokenKind::ArithmeticOperator(ArithmeticOps::Asterisk),
            "*",
        ))
    }
}

/// Tokenize `/` or `/=`
fn lex_slash(stream: &mut CharStream) -> Result<Token, LexError> {
    let is_assign = stream.peek_n(1) == Some(b'=');
    let builder = TokenBuilder::new(stream);
    if is_assign {
        Ok(builder.multi_char_token(
            2,
            TokenKind::AssignmentOperator(AssignmentOps::DivideAssign),
            "/=",
        ))
    } else {
        Ok(builder.single_char_token(
            TokenKind::ArithmeticOperator(ArithmeticOps::Slash),
            "/",
        ))
    }
}

/// Tokenize `%` or `%=`
fn lex_modulo(stream: &mut CharStream) -> Result<Token, LexError> {
    let is_assign = stream.peek_n(1) == Some(b'=');
    let builder = TokenBuilder::new(stream);
    if is_assign {
        Ok(builder.multi_char_token(
            2,
            TokenKind::AssignmentOperator(AssignmentOps::ModuloAssign),
            "%=",
        ))
    } else {
        Ok(builder.single_char_token(
            TokenKind::ArithmeticOperator(ArithmeticOps::Modulo),
            "%",
        ))
    }
}

/// Tokenize `<`, `<=`, or `<<`
fn lex_less_than(stream: &mut CharStream) -> Result<Token, LexError> {
    let next = stream.peek_n(1);
    let builder = TokenBuilder::new(stream);
    if next == Some(b'=') {
        Ok(builder.multi_char_token(
            2,
            TokenKind::RelationalOperator(RelationalOps::LessThanOrEqual),
            "<=",
        ))
    } else if next == Some(b'<') {
        Ok(builder.multi_char_token(
            2,
            TokenKind::BitwiseOperator(BitwiseOps::LeftShift),
            "<<",
        ))
    } else {
        Ok(builder.single_char_token(
            TokenKind::RelationalOperator(RelationalOps::LessThan),
            "<",
        ))
    }
}

/// Tokenize `>`, `>=`, or `>>`
fn lex_greater_than(stream: &mut CharStream) -> Result<Token, LexError> {
    let next = stream.peek_n(1);
    let builder = TokenBuilder::new(stream);
    if next == Some(b'=') {
        Ok(builder.multi_char_token(
            2,
            TokenKind::RelationalOperator(RelationalOps::GreaterThanOrEqual),
            ">=",
        ))
    } else if next == Some(b'>') {
        Ok(builder.multi_char_token(
            2,
            TokenKind::BitwiseOperator(BitwiseOps::RightShift),
            ">>",
        ))
    } else {
        Ok(builder.single_char_token(
            TokenKind::RelationalOperator(RelationalOps::GreaterThan),
            ">",
        ))
    }
}

/// Tokenize `!` or `!=`
fn lex_not(stream: &mut CharStream) -> Result<Token, LexError> {
    let is_not_equal = stream.peek_n(1) == Some(b'=');
    let builder = TokenBuilder::new(stream);
    if is_not_equal {
        Ok(builder.multi_char_token(
            2,
            TokenKind::RelationalOperator(RelationalOps::NotEqual),
            "!=",
        ))
    } else {
        Ok(builder.single_char_token(
            TokenKind::LogicalOperator(LogicalOps::Not),
            "!",
        ))
    }
}

/// Tokenize `&` or `&&`
fn lex_ampersand(stream: &mut CharStream) -> Result<Token, LexError> {
    let is_logical = stream.peek_n(1) == Some(b'&');
    let builder = TokenBuilder::new(stream);
    if is_logical {
        Ok(builder.multi_char_token(
            2,
            TokenKind::LogicalOperator(LogicalOps::And),
            "&&",
        ))
    } else {
        Ok(builder.single_char_token(
            TokenKind::BitwiseOperator(BitwiseOps::And),
            "&",
        ))
    }
}

/// Tokenize `|` (bitwise OR) or `||` (logical OR).
///
/// # Returns
///
/// - `||` → `LogicalOperator::Or`
/// - `|` → `BitwiseOperator::Or`
fn lex_pipe(stream: &mut CharStream) -> Result<Token, LexError> {
    let is_logical = stream.peek_n(1) == Some(b'|');
    let builder = TokenBuilder::new(stream);
    if is_logical {
        Ok(builder.multi_char_token(
            2,
            TokenKind::LogicalOperator(LogicalOps::Or),
            "||",
        ))
    } else {
        Ok(builder.single_char_token(
            TokenKind::BitwiseOperator(BitwiseOps::Or),
            "|",
        ))
    }
}

