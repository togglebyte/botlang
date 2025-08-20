use std::num::{ParseFloatError, ParseIntError};

use crate::lexer::Token;
use crate::op::Op;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UnterminatedString,
    NotEnoughCake(Token),
    IntError(ParseIntError),
    FloatError(ParseFloatError),
    MissingClosingParen,
    MissingComma,
    InvalidToken(Token),
    InvalidOperator(Op),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnterminatedString => write!(f, "unterminated string"),
            Self::NotEnoughCake(token) => write!(f, "not enough cake: {token:?}"),
            Self::IntError(parse_int_error) => write!(f, "{parse_int_error}"),
            Self::FloatError(parse_float_error) => write!(f, "{parse_float_error}"),
            Self::MissingClosingParen => write!(f, "missing closing paren"),
            Self::MissingComma => write!(f, "missing a comma in a collection"),
            Self::InvalidToken(token) => write!(f, "invalid token: {token:?}"),
            Self::InvalidOperator(op) => write!(f, "invalid operator: {op:?}"),
        }
    }
}

impl std::error::Error for Error {
}

impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Self {
        Self::IntError(e)
    }
}

impl From<ParseFloatError> for Error {
    fn from(e: ParseFloatError) -> Self {
        Self::FloatError(e)
    }
}
