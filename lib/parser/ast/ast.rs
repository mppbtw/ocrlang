use std::fmt::Debug;
use std::fmt::Display;

use super::expressions::Expression;
use super::expressions::Identifier;
use crate::lexer::Token;

pub trait AstNode: Display {}

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
        value:  Option<Expression<'a>>,
    },
    Return {
        token: Token<'a>,
        value: Option<Expression<'a>>,
    },
    #[default]
    Empty,
}
