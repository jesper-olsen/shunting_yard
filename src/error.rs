use std::fmt;
use std::io;

use crate::scanner::Token;
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Custom(String),
    IO(io::Error),
    MismatchedParentheses,
    ExpectedNumberOnStack,
    BadFunctionCall(Box<Error>),
    UnknownFunction(Token),
    UnknownFunctionType,
    BadExpression,
    UnexpectedChar(char),
    ErrorParseNumber(String),
}
impl Error {
    pub fn custom(val: impl std::fmt::Display) -> Self {
        Self::custom(val.to_string())
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
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}
