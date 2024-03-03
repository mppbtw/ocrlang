use std::fmt;

use crate::lexer::Token;

pub trait Expression {}

#[derive(Default)]
pub struct Program<'a> {
    pub statements: Vec<Statement<'a>>,
}

#[derive(Default)]
pub enum Statement<'a> {
    Assign {
        token: Token<'a>,
        ident: Identifier<'a>,
        value: Option<Box<dyn Expression>>,
    },
    #[default]
    Empty,
}

#[derive(Debug, Clone)]
pub struct Identifier<'a> {
    /// Token::Ident
    token: Token<'a>,
}
impl<'a> Identifier<'a> {
    pub fn new(token: Token<'a>) -> Self {
        Self { token }
    }
}

impl Expression for Identifier<'_> {}
