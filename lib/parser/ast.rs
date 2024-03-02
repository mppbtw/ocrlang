use std::fmt;

use crate::lexer::Token;

pub trait Expression{}

#[derive(Default)]
pub struct Program<'a> {
    pub statements: Vec<Statement<'a>>,
}

pub enum Statement<'a> {
    AssignStatement {
        token: Token<'a>,
        ident: Identifier,
        value: Box<dyn Expression>,
    },
}


pub struct Identifier {}

pub struct AssignStatementData {

}
