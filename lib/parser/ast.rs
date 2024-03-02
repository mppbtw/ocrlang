use std::fmt;

use crate::lexer::Token;

#[derive(Debug, Clone)]
pub struct Expression {}

#[derive(Default)]
pub struct Program<'a> {
    pub statements: Vec<Statement<'a>>,
}

#[derive(Default, Clone)]
pub enum Statement<'a> {
    Assign {
        token: Token<'a>,
        ident: Identifier,
        value: Expression,
    },
    #[default]
    Empty,
}

#[derive(Debug, Clone)]
pub struct Identifier {}

pub struct AssignStatementData {}
