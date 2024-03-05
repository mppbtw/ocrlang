use std::fmt::Debug;

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
