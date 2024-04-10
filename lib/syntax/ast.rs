use std::fmt::Debug;
use std::fmt::Display;

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
            + self.ident.get_ident()
            + " = "
            + &self.value.pretty_print())
            .to_owned()
    }
}
impl AstNode for AssignStatement<'_> {}
impl Statement for AssignStatement<'_> {
    fn get_type(&self) -> StatementType {
        StatementType::Assign(self)
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
        StatementType::Return(self)
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
        StatementType::Expression(self)
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

pub enum ExpressionType<'a> {
    Identifier(&'a Identifier<'a>),
    Boolean(&'a BooleanExpression<'a>),
    Placeholder(&'a PlaceholderExpression),
    IntegerLiteral(&'a IntegerLiteralExpression<'a>),
    Prefix(&'a PrefixExpression<'a>),
    Infix(&'a InfixExpression<'a>),
}

pub trait Expression: AstNode {
    fn get_type(&self) -> ExpressionType;
    /// Instead of `1 + 2 * 3` will give `(1 + (2 * 3))`
    fn pretty_print_with_brackets(&self) -> String {
        self.pretty_print()
    }
}
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
impl Expression for Identifier<'_> {
    fn get_type(&self) -> ExpressionType {
        ExpressionType::Identifier(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum InfixOperator {
    Plus,
    Minus,
    Divide,
    Div,
    Mod,
    Multiply,
    DoubleEquals,
    NotEqual,
    LThan,
    LThanOrEqual,
    GThanOrEqual,
    GThan,
    Or,
}
impl TryFrom<Token<'_>> for InfixOperator {
    type Error = NoSuchInfixOperatorError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        use Token::*;

        match value {
            Plus => Ok(Self::Plus),
            Minus => Ok(Self::Minus),
            FSlash => Ok(Self::Divide),
            Div => Ok(Self::Div),
            Mod => Ok(Self::Mod),
            Asterisk => Ok(Self::Multiply),
            DoubleEquals => Ok(Self::DoubleEquals),
            LThan => Ok(Self::LThan),
            LThanOrEqual => Ok(Self::LThanOrEqual),
            GThan => Ok(Self::GThan),
            GThanOrEqual => Ok(Self::GThanOrEqual),
            NotEqual => Ok(Self::NotEqual),
            Or => Ok(Self::Or),
            _ => Err(NoSuchInfixOperatorError),
        }
    }
}
impl Display for InfixOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Div => " DIV ",
            Self::Mod => " MOD ",
            Self::Or => " OR ",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Divide => "/",
            Self::Multiply => "*",
            Self::DoubleEquals => "==",
            Self::LThanOrEqual => "<=",
            Self::LThan => "<",
            Self::GThan => ">",
            Self::GThanOrEqual => ">=",
            Self::NotEqual => "!=",
        })
    }
}
#[derive(Debug, Clone)]
pub struct NoSuchInfixOperatorError;

#[derive(Debug)]
pub struct InfixExpression<'a> {
    pub token:    Token<'a>,
    pub operator: InfixOperator,
    pub left:     Box<dyn Expression + 'a>,
    pub right:    Box<dyn Expression + 'a>,
}
impl PrettyPrint for InfixExpression<'_> {
    fn pretty_print(&self) -> String {
        self.left.pretty_print() + &self.operator.to_string() + &self.right.pretty_print()
    }
}
impl AstNode for InfixExpression<'_> {}
impl Expression for InfixExpression<'_> {
    fn get_type(&self) -> ExpressionType {
        ExpressionType::Infix(self)
    }

    fn pretty_print_with_brackets(&self) -> String {
        "(".to_owned()
            + &self.left.pretty_print_with_brackets()
            + &self.operator.to_string()
            + &self.right.pretty_print_with_brackets()
            + ")"
    }
}

#[derive(Debug)]
pub enum PrefixOperator {
    Plus,
    Minus,
    Not,
}
impl TryFrom<Token<'_>> for PrefixOperator {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Plus => Ok(Self::Plus),
            Token::Minus => Ok(Self::Minus),
            Token::Not => Ok(Self::Not),
            _ => Err(()),
        }
    }
}
impl Display for PrefixOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Not => "NOT ",
        })
    }
}

#[derive(Debug)]
pub struct PrefixExpression<'a> {
    pub token:    Token<'a>,
    pub operator: PrefixOperator,
    pub subject:  Box<dyn Expression + 'a>,
}
impl PrettyPrint for PrefixExpression<'_> {
    fn pretty_print(&self) -> String {
        self.operator.to_string() + &self.subject.pretty_print()
    }
}
impl AstNode for PrefixExpression<'_> {}
impl Expression for PrefixExpression<'_> {
    fn get_type(&self) -> ExpressionType {
        ExpressionType::Prefix(self)
    }
    fn pretty_print_with_brackets(&self) -> String {
        "(".to_owned() + &self.pretty_print() + ")"
    }
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
impl Expression for IntegerLiteralExpression<'_> {
    fn get_type(&self) -> ExpressionType {
        ExpressionType::IntegerLiteral(self)
    }
}

#[derive(Debug)]
pub struct PlaceholderExpression {}
impl PrettyPrint for PlaceholderExpression {
    fn pretty_print(&self) -> String {
        "<PLACEHOLDER_EXPRESSION>".to_owned()
    }
}
impl AstNode for PlaceholderExpression {}
impl Expression for PlaceholderExpression {
    fn get_type(&self) -> ExpressionType {
        ExpressionType::Placeholder(self)
    }
}

#[derive(Debug)]
pub struct BooleanExpression<'a> {
    pub token: Token<'a>,
    pub value: bool,
}
impl PrettyPrint for BooleanExpression<'_> {
    fn pretty_print(&self) -> String {
        if self.value { "true" } else { "false" }.to_owned()
    }
}
impl AstNode for BooleanExpression<'_> {}
impl Expression for BooleanExpression<'_> {
    fn get_type(&self) -> ExpressionType {
        ExpressionType::Boolean(self)
    }
}
