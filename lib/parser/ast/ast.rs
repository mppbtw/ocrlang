use std::fmt::Debug;

use super::expressions::Expression;
use super::expressions::Identifier;
use crate::lexer::Token;

/// Meta-trait for all of the stuff needed in AST statements/expressions
pub trait AstNode: PrettyPrint + Debug {}

/// Format the code in human readable form
pub trait PrettyPrint {
    fn pretty_print(&self) -> String;
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
