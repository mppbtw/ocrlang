use std::fmt::Debug;
use std::fmt::{self};

use crate::lexer::Token;

pub trait Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}
impl Debug for dyn Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}

#[derive(Default)]
pub struct Program<'a> {
    pub statements: Vec<Statement<'a>>,
}

#[derive(Default, Debug)]
pub enum Statement<'a> {
    Assign {
        token:  Token<'a>,
        ident:  Identifier<'a>,
        global: bool,
        value:  Option<Box<dyn Expression>>,
    },
    Return {
        token: Token<'a>,
        value: Option<Box<dyn Expression>>,
    },
    #[default]
    Empty,
}

#[derive(Debug, Clone)]
pub struct Identifier<'a> {
    /// Token::Ident
    pub token: Token<'a>,
}
impl<'a> Identifier<'a> {
    pub fn new(token: Token<'a>) -> Self {
        Self { token }
    }

    pub fn compare_ident(&self, other: &str) -> bool {
        match self.token {
            Token::Identifier(i) => i == other,
            _ => false,
        }
    }

    pub fn get_ident(&self) -> &'a str {
        match self.token {
            Token::Identifier(i) => i,
            _ => "",
        }
    }
}

impl Expression for Identifier<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "lol")
    }
}
