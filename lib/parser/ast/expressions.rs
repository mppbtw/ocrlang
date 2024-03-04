use std::fmt;
use std::fmt::Display;

use super::ast::AstNode;
use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Expression<'a> {
    Identifier(Identifier<'a>),
}
impl Display for Expression<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}
impl AstNode for Expression<'_> {}

#[derive(Debug, Clone)]
pub struct Identifier<'a> {
    /// Will always be `Token::Ident`
    pub token: Token<'a>,
}
impl Identifier<'_> {
    pub fn get_ident(&self) -> &str {
        match self.token {
            Token::Identifier(i) => i,
            _ => unreachable!(),
        }
    }
}
impl AstNode for Identifier<'_> {}
impl Display for Identifier<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "An identifier: {}", self.get_ident())
    }
}
