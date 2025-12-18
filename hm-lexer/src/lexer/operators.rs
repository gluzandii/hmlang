//! Operator tokenization logic.
//!
//! This module handles tokenizing all operators including arithmetic,
//! assignment, relational, logical, and bitwise operators. It consolidates
//! the repetitive operator-matching logic from the main lexer.

use crate::charstream::CharStream;
use crate::lexerror::LexError;
use crate::token::operators::arithmetic::ArithmeticOperator;
use crate::token::operators::assignment::AssignmentOperator;
use crate::token::operators::bitwise::BitwiseOperator;
use crate::token::operators::logical::LogicalOperator;
use crate::token::operators::relational::RelationalOperator;
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
                TokenKind::BitwiseOperator(BitwiseOperator::Xor),
                "^",
            ))
        }
        b'~' => {
            let builder = TokenBuilder::new(stream);
            Ok(builder.single_char_token(
                TokenKind::BitwiseOperator(BitwiseOperator::Not),
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
            TokenKind::RelationalOperator(RelationalOperator::Equal),
            "==",
        ))
    } else {
        Ok(builder.single_char_token(
            TokenKind::AssignmentOperator(AssignmentOperator::Assign),
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
            TokenKind::AssignmentOperator(AssignmentOperator::AddAssign),
            "+=",
        ))
    } else {
        Ok(builder.single_char_token(
            TokenKind::ArithmeticOperator(ArithmeticOperator::Plus),
            "+",
        ))
    }
}

/// Tokenize `-` or `-=`
fn lex_minus(stream: &mut CharStream) -> Result<Token, LexError> {
    let is_assign = stream.peek_n(1) == Some(b'=');
    let builder = TokenBuilder::new(stream);
    if is_assign {
        Ok(builder.multi_char_token(
            2,
            TokenKind::AssignmentOperator(AssignmentOperator::SubtractAssign),
            "-=",
        ))
    } else {
        Ok(builder.single_char_token(
            TokenKind::ArithmeticOperator(ArithmeticOperator::Minus),
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
            TokenKind::AssignmentOperator(AssignmentOperator::MultiplyAssign),
            "*=",
        ))
    } else if next == Some(b'*') {
        Ok(builder.multi_char_token(
            2,
            TokenKind::ArithmeticOperator(ArithmeticOperator::Exponent),
            "**",
        ))
    } else {
        Ok(builder.single_char_token(
            TokenKind::ArithmeticOperator(ArithmeticOperator::Asterisk),
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
            TokenKind::AssignmentOperator(AssignmentOperator::DivideAssign),
            "/=",
        ))
    } else {
        Ok(builder.single_char_token(
            TokenKind::ArithmeticOperator(ArithmeticOperator::Slash),
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
            TokenKind::AssignmentOperator(AssignmentOperator::ModuloAssign),
            "%=",
        ))
    } else {
        Ok(builder.single_char_token(
            TokenKind::ArithmeticOperator(ArithmeticOperator::Modulo),
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
            TokenKind::RelationalOperator(RelationalOperator::LessThanOrEqual),
            "<=",
        ))
    } else if next == Some(b'<') {
        Ok(builder.multi_char_token(
            2,
            TokenKind::BitwiseOperator(BitwiseOperator::LeftShift),
            "<<",
        ))
    } else {
        Ok(builder.single_char_token(
            TokenKind::RelationalOperator(RelationalOperator::LessThan),
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
            TokenKind::RelationalOperator(RelationalOperator::GreaterThanOrEqual),
            ">=",
        ))
    } else if next == Some(b'>') {
        Ok(builder.multi_char_token(
            2,
            TokenKind::BitwiseOperator(BitwiseOperator::RightShift),
            ">>",
        ))
    } else {
        Ok(builder.single_char_token(
            TokenKind::RelationalOperator(RelationalOperator::GreaterThan),
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
            TokenKind::RelationalOperator(RelationalOperator::NotEqual),
            "!=",
        ))
    } else {
        Ok(builder.single_char_token(
            TokenKind::LogicalOperator(LogicalOperator::Not),
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
            TokenKind::LogicalOperator(LogicalOperator::And),
            "&&",
        ))
    } else {
        Ok(builder.single_char_token(
            TokenKind::BitwiseOperator(BitwiseOperator::And),
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
            TokenKind::LogicalOperator(LogicalOperator::Or),
            "||",
        ))
    } else {
        Ok(builder.single_char_token(
            TokenKind::BitwiseOperator(BitwiseOperator::Or),
            "|",
        ))
    }
}

