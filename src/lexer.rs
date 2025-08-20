use std::iter::Peekable;
use std::str::Chars;

use crate::error::{Error, Result};
use crate::op::Op;

#[derive(Debug)]
pub struct Tokens {
    inner: Vec<Token>,
    index: usize,
}

impl Tokens {
    pub fn empty() -> Self {
        Self {
            inner: vec![],
            index: 0,
        }
    }

    pub(crate) fn consume(&mut self) {
        if self.index == self.inner.len() {
            return;
        }
        self.index += 1;
    }

    pub(crate) fn take(&mut self) -> Token {
        if self.index == self.inner.len() {
            return Token::Eof;
        }
        self.index += 1;
        self.inner[self.index - 1].consume()
    }

    fn push(&mut self, token: Token) {
        self.inner.push(token);
    }

    pub fn peek(&self) -> &Token {
        if self.index == self.inner.len() {
            return &Token::Eof;
        }
        &self.inner[self.index]
    }

    pub(crate) fn take_string(&mut self) -> String {
        let token = self.inner[self.index].consume();
        self.index += 1;
        match token {
            Token::Ident(ident) => ident,
            Token::Str(s) => s,
            token => panic!("invalid token: {token:?}"),
        }
    }

    pub(crate) fn take_int(&mut self) -> i64 {
        let token = self.inner[self.index].consume();
        self.index += 1;
        match token {
            Token::Int(int) => int,
            _ => unreachable!("invalid token"),
        }
    }

    pub(crate) fn take_float(&mut self) -> f64 {
        let token = self.inner[self.index].consume();
        self.index += 1;
        match token {
            Token::Float(f) => f,
            _ => unreachable!("invalid token"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LCurly,
    RCurly,
    Comma,
    SemiColon,
    EqualEqual,
    Op(Op),
    DotDot,
    Let,
    Ident(String),
    Str(String),
    Int(i64),
    Float(f64),

    // Consumed tokens should never be generated.
    // They are an artifact of consuming a token, leaving
    // something behind
    Consumed,

    Eof,
}

impl Token {
    pub fn consume(&mut self) -> Self {
        let mut consumed = Self::Consumed;
        std::mem::swap(&mut consumed, self);
        consumed
    }
}

pub fn lex(src: &str) -> Result<Tokens> {
    Lexer::new(src).lex()
}

struct Lexer<'src> {
    chars: Peekable<Chars<'src>>,
    tokens: Tokens,
}

impl<'src> Lexer<'src> {
    fn new(src: &'src str) -> Self {
        Self {
            chars: src.chars().peekable(),
            tokens: Tokens::empty(),
        }
    }

    fn consume(&mut self) -> () {
        _ = self.chars.next();
    }

    fn lex(mut self) -> Result<Tokens> {
        loop {
            let Some(c) = self.chars.next() else { return Ok(self.tokens) };
            let next = self.chars.peek().copied();

            match c {
                '.' if next == Some('.') => self.push(Token::DotDot),
                '=' if next == Some('=') => self.push(Token::EqualEqual),
                '=' => self.push(Token::Op(Op::Equal)),
                ',' => self.push(Token::Comma),
                '(' => self.push(Token::Op(Op::LParen)),
                ')' => self.push(Token::Op(Op::RParen)),
                '[' => self.push(Token::Op(Op::LBracket)),
                ']' => self.push(Token::Op(Op::RBracket)),
                '+' => self.push(Token::Op(Op::Plus)),
                '-' => self.push(Token::Op(Op::Minus)),
                '*' => self.push(Token::Op(Op::Mul)),
                '/' => self.push(Token::Op(Op::Div)),
                ';' => self.push(Token::SemiColon),
                '{' => self.push(Token::LCurly),
                '}' => self.push(Token::RCurly),
                ident @ ('a'..='z' | 'A'..='Z') => self.ident(ident)?,
                num @ ('0'..='9') => self.number(num)?,
                '"' => self.string()?,
                // _ if c.is_whitespace() => continue,
                _ => continue,
            }
        }
    }

    fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn string(&mut self) -> Result<()> {
        let mut buffer = String::new();

        loop {
            match self.chars.peek() {
                Some('"') => {
                    self.consume();
                    self.tokens.push(Token::Str(buffer));
                    break Ok(());
                }
                Some(c) => {
                    buffer.push(*c);
                    self.consume();
                }
                None => break Err(Error::UnterminatedString),
            }
        }
    }

    fn ident(&mut self, start: char) -> Result<()> {
        let mut buffer = String::from(start);

        loop {
            match self.chars.peek() {
                Some(c @ ('a'..='z' | 'A'..='Z' | '_')) => {
                    buffer.push(*c);
                    self.consume();
                }
                None | Some(_) => break,
            }
        }

        let token = match buffer.as_str() {
            "let" => Token::Let,
            _ => Token::Ident(buffer),
        };

        self.tokens.push(token);
        Ok(())
    }

    fn number(&mut self, num: char) -> Result<()> {
        let mut buffer = String::from(num);
        let mut is_float = false;

        loop {
            match self.chars.peek() {
                Some(c @ ('0'..='9')) => {
                    buffer.push(*c);
                    self.consume();
                }
                Some('.') if !is_float => {
                    buffer.push('.');
                    self.consume();
                    is_float = true;
                }
                None | Some(_) => break,
            }
        }

        match is_float {
            true => {
                let float = buffer.parse::<f64>()?;
                self.tokens.push(Token::Float(float));
            }
            false => {
                let int = buffer.parse::<i64>()?;
                self.tokens.push(Token::Int(int));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn string() {
        let input = "\"hello world\"";
        let mut tokens = lex(input).unwrap();
        assert_eq!(Token::Str("hello world".into()), tokens.inner.remove(0));
    }

    #[test]
    fn numberwang() {
        let input = "123";
        let mut tokens = lex(input).unwrap();
        assert_eq!(Token::Int(123), tokens.inner.remove(0));
    }
}
