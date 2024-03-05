use std::fmt::Debug;

use super::ast::AstNode;
use super::ast::PrettyPrint;
use crate::lexer::Token;

pub trait Expression: AstNode {}

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
impl PrettyPrint for Identifier<'_> {
    fn pretty_print(&self) -> String {
        self.get_ident().to_owned()
    }
}
impl AstNode for Identifier<'_> {}
impl Expression for Identifier<'_> {}
