use std::mem;
use std::str::Chars;

use OperatorType::*;

use crate::error::{Error, Result};

#[derive(PartialEq, Debug)]
pub enum OperatorType {
    Plus,
    Minus,
    Multiply,
    Divide,
    Pow,
}

impl OperatorType {
    pub fn precedence(&self) -> u8 {
        use OperatorType::*;
        match self {
            Plus | Minus => 2,
            Multiply | Divide => 3,
            Pow => 4,
        }
    }

    pub fn is_left_associative(&self) -> bool {
        use OperatorType::*;
        matches!(self, Plus | Minus | Multiply | Divide)
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Function(String),
    Operator(OperatorType),
    Comma,
    LeftParen,
    RightParen,
}

// A peekable iterator for the program text...
struct Text<'a> {
    pos: Chars<'a>,
    look_ahead: [Option<char>; 2],
}

impl Text<'_> {
    fn new(source: &str) -> Text<'_> {
        let mut txt = Text {
            pos: source.chars(),
            look_ahead: [None, None],
        };
        txt.look_ahead
            .iter_mut()
            .for_each(|it| *it = txt.pos.next());
        txt
    }

    fn peek(&self, offset: usize) -> Option<char> {
        self.look_ahead[offset]
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.look_ahead[0];
        self.look_ahead[0] = self.look_ahead[1];
        self.look_ahead[1] = self.pos.next();
        ch
    }
}

pub struct Scanner<'a> {
    text: Text<'a>,
    lexeme: String,
    line: u32,
}

impl Scanner<'_> {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            text: Text::new(source),
            lexeme: String::new(),
            line: 1,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.text.advance();
        if let Some(c) = ch {
            self.lexeme.push(c)
        }
        ch
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.text.peek(0) {
                Some(' ' | '\r' | '\t') => _ = self.text.advance(),
                Some('\n') => {
                    self.line += 1;
                    self.text.advance();
                }
                Some('/') => {
                    // comment
                    let Some('/') = self.text.peek(1) else {
                        return;
                    };
                    while let Some(ch) = self.text.peek(0) {
                        if ch == '\n' {
                            break;
                        }
                        self.text.advance();
                    }
                }
                _ => return,
            }
        }
    }

    fn parse_digits(&mut self) {
        while let Some('0'..='9') = self.text.peek(0) {
            self.advance();
        }
    }

    fn number(&mut self) -> Result<Token> {
        self.parse_digits();
        if let Some('.') = self.text.peek(0) {
            if let Some('0'..='9') = self.text.peek(1) {
                self.advance(); // Consume '.'
                self.parse_digits();
            }
        }

        let n = self
            .lexeme
            .parse::<f64>()
            .map_err(|_| Error::ParseNumber(self.lexeme.to_string()))?;

        Ok(Token::Number(n))
    }

    fn identifier(&mut self) -> Result<Token> {
        while let Some(ch) = self.text.peek(0) {
            if is_alpha(ch) || ch.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }
        match self.lexeme.as_str() {
            "min" | "max" | "sin" | "cos" => Ok(Token::Function(mem::take(&mut self.lexeme))),
            _ => Err(Error::UnknownFunction(mem::take(&mut self.lexeme))),
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        loop {
            self.skip_whitespace();
            let Some(ch) = self.advance() else {
                return Ok(tokens);
            };
            tokens.push(match ch {
                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                ',' => Token::Comma,
                '-' => Token::Operator(Minus),
                '+' => Token::Operator(Plus),
                '*' => Token::Operator(Multiply),
                '/' => Token::Operator(Divide),
                '^' => Token::Operator(Pow),
                _ if is_alpha(ch) => self.identifier()?,
                _ if ch.is_ascii_digit() => self.number()?,
                _ => return Err(Error::UnexpectedChar(ch)),
            });
            self.lexeme.clear();
        }
    }
}

const fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}
