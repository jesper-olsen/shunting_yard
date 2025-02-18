use crate::scanner::Token;
use std::fmt;
use std::io;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Custom(String),
    IO(io::Error),
    MismatchedParentheses,
    ExpectedNumberOnStack,
    UnknownFunction(String),
    BadExpression,
    UnexpectedChar(char),
    ParseNumber(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom(msg) => write!(f, "{}", msg),
            Self::IO(err) => write!(f, "I/O error: {}", err),
            Self::MismatchedParentheses => write!(f, "Mismatched parentheses"),
            Self::ExpectedNumberOnStack => write!(f, "Expected number on stack"),
            Self::UnknownFunction(s) => write!(f, "Unknown function: {s}"),
            Self::BadExpression => write!(f, "Bad Expression"),
            Self::UnexpectedChar(c) => write!(f, "Unexpected character: {c}"),
            Self::ParseNumber(s) => write!(f, "Failed to parse number: {s}"),
        }
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Custom(value.to_string())
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}
