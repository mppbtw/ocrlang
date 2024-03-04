use std::fmt;
use std::fmt::Debug;

use crate::lexer::Token;

#[derive(Default)]
pub struct Program<'a> {
    pub statements: Vec<Statement<'a>>,
}

#[derive(Debug, Clone)]
pub enum Expression<'a> {
    Identifier(Identifier<'a>),
}

#[derive(Debug, Clone)]
pub struct Identifier<'a> {
    /// Will always be `Token::Ident`
    pub token: Token<'a>,
}
impl Identifier<'_> {
    pub fn get_ident(&self) -> &str {
        match self.token {
            Token::Identifier(i) => i,
            _ => unreachable!()
        }
    }
}

#[derive(Default, Debug)]
pub enum Statement<'a> {
    Assign {
        token:  Token<'a>,
        ident:  Identifier<'a>,
        global: bool,
        value:  Option<Expression<'a>>,
    },
    Return {
        token: Token<'a>,
        value: Option<Expression<'a>>,
    },
    #[default]
    Empty,
}
