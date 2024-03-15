use std::fmt::Debug;

use crate::lexer::Token;

/// Meta-trait for all of the stuff needed in AST statements/expressions
pub trait AstNode: PrettyPrint + Debug {}

/// Format the code in human readable form
pub trait PrettyPrint {
    fn pretty_print(&self) -> String;
}

pub trait Statement: AstNode {
    fn get_type(&self) -> StatementType;
}

pub enum StatementType<'a> {
    Assign(&'a AssignStatement<'a>),
    Return(&'a ReturnStatement<'a>),
    Expression(&'a ExpressionStatement<'a>),
    Empty,
}

#[derive(Debug)]
pub struct AssignStatement<'a> {
    pub token:  Token<'a>,
    pub ident:  Identifier<'a>,
    pub global: bool,
    pub value:  Box<dyn Expression + 'a>,
}
impl PrettyPrint for AssignStatement<'_> {
    fn pretty_print(&self) -> String {
        (if self.global { "global " } else { "" }.to_owned()
            + &self.ident.get_ident()
            + " = "
            + &self.value.pretty_print())
            .to_owned()
    }
}
impl AstNode for AssignStatement<'_> {}
impl Statement for AssignStatement<'_> {
    fn get_type(&self) -> StatementType {
        StatementType::Assign(&self)
    }
}

#[derive(Debug)]
pub struct ReturnStatement<'a> {
    pub token: Token<'a>,
    pub value: Option<Box<dyn Expression + 'a>>,
}
impl PrettyPrint for ReturnStatement<'_> {
    fn pretty_print(&self) -> String {
        match &self.value {
            Some(v) => "return ".to_owned() + &v.pretty_print(),
            None => "return".to_owned(),
        }
        .to_owned()
    }
}
impl AstNode for ReturnStatement<'_> {}
impl Statement for ReturnStatement<'_> {
    fn get_type(&self) -> StatementType {
        StatementType::Return(&self)
    }
}

#[derive(Debug)]
pub struct ExpressionStatement<'a> {
    pub value: Box<dyn Expression + 'a>,
}
impl PrettyPrint for ExpressionStatement<'_> {
    fn pretty_print(&self) -> String {
        self.value.pretty_print()
    }
}
impl AstNode for ExpressionStatement<'_> {}
impl Statement for ExpressionStatement<'_> {
    fn get_type(&self) -> StatementType {
        StatementType::Expression(&self)
    }
}

#[derive(Debug)]
pub struct EmptyStatement {}
impl PrettyPrint for EmptyStatement {
    fn pretty_print(&self) -> String {
        String::new()
    }
}
impl AstNode for EmptyStatement {}
impl Statement for EmptyStatement {
    fn get_type(&self) -> StatementType {
        StatementType::Empty
    }
}

pub trait Expression: AstNode {}
impl Default for Box<dyn Expression> {
    fn default() -> Self {
        Box::new(Identifier {
            token: Token::Identifier("lol"),
        })
    }
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

#[derive(Debug, Clone)]
pub enum InfixOperator {
    Plus,
    Minus,
    Divide,
    Div,
    Mod,
    Multiply,
    FunctionCall,
}
impl TryFrom<Token<'_>> for InfixOperator {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Plus => Ok(Self::Plus),
            Token::Minus => Ok(Self::Minus),
            Token::FSlash => Ok(Self::Divide),
            Token::Div => Ok(Self::Div),
            Token::Mod => Ok(Self::Mod),
            Token::Asterisk => Ok(Self::Multiply),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct InfixExpression<'a> {
    pub token:    Token<'a>,
    pub operator: InfixOperator,
    pub left:     Box<dyn Expression + 'a>,
    pub right:    Box<dyn Expression + 'a>,
}

#[derive(Debug)]
pub struct IntegerLiteralExpression<'a> {
    pub token: Token<'a>,
    pub value: i128,
}
impl PrettyPrint for IntegerLiteralExpression<'_> {
    fn pretty_print(&self) -> String {
        format!("{}", self.value)
    }
}
impl AstNode for IntegerLiteralExpression<'_> {}
impl Expression for IntegerLiteralExpression<'_> {}

#[derive(Debug)]
pub struct PlaceholderExpression {}
impl PrettyPrint for PlaceholderExpression {
    fn pretty_print(&self) -> String {
        "<--PLACEHOLDEREXPRESSION-->".to_owned()
    }
}
impl AstNode for PlaceholderExpression {}
impl Expression for PlaceholderExpression {}
