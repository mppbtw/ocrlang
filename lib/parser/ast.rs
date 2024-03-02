use crate::lexer::Token;

pub trait Node {}
pub trait Expression {}
pub trait Statement {
    fn get_tagged_token(&self) -> Token;
}

#[derive(Default)]
pub struct Program {
    pub statements: Vec<Box<dyn Statement>>,
}

pub struct Identifier {}
impl Expression for Identifier {}

pub struct VarAssignStatement<'a> {
    tagged_token: Token<'a>,
    pub ident:    Identifier,
    pub value:    Box<dyn Expression>,
}

impl Statement for VarAssignStatement<'_> {
    /// Either the 'global' keyword or the identifier itself
    fn get_tagged_token(&self) -> Token {
        self.tagged_token
    }
}
