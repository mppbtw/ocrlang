use crate::lexer::Lexer;
use crate::lexer::LexerError;
use crate::lexer::Token;
use crate::syntax::AssignStatement;
use crate::syntax::BooleanExpression;
use crate::syntax::Expression;
use crate::syntax::ExpressionStatement;
use crate::syntax::Identifier;
use crate::syntax::InfixExpression;
use crate::syntax::IntegerLiteralExpression;
use crate::syntax::NoSuchInfixOperatorError;
use crate::syntax::PlaceholderExpression;
use crate::syntax::PrefixExpression;
use crate::syntax::ReturnStatement;
use crate::syntax::Statement;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    #[default]
    Lowest,
    Equality,
    Inequality,
    And,
    Or,
    Sum,
    Product,
    Prefix,
    Call,
}
impl From<Token<'_>> for Precedence {
    fn from(value: Token) -> Self {
        use Token::*;

        match value {
            GThan | LThan | LThanOrEqual | GThanOrEqual => Self::Inequality,
            DoubleEquals | NotEqual => Self::Equality,
            Plus | Minus | Mod | Div => Self::Sum,
            FSlash | Asterisk => Self::Product,
            And => Self::And,
            Or => Self::Or,
            _ => Self::Lowest,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ParserError {
    UnterminatedStringLiteral,
    InvalidNumberLiteral,
    TooLargeInteger,

    #[default]
    UnexpectedToken,
}
impl From<LexerError> for ParserError {
    fn from(value: LexerError) -> Self {
        match value {
            LexerError::UnterminatedStringLiteral => Self::UnterminatedStringLiteral,
        }
    }
}
impl From<NoSuchInfixOperatorError> for ParserError {
    fn from(_: NoSuchInfixOperatorError) -> Self {
        ParserError::UnexpectedToken
    }
}

#[derive(Default, Debug)]
pub struct Program<'a> {
    pub statements: Vec<Box<dyn Statement + 'a>>,
}

#[derive(Default, Debug)]
struct Parser<'a> {
    lexer:    Lexer<'a>,
    tok:      Token<'a>,
    peek_tok: Token<'a>,
    pub prog: Program<'a>,
}

impl<'a> Parser<'a> {
    pub fn parse(&mut self) -> Result<(), ParserError> {
        loop {
            match () {
                () if matches!(self.tok, Token::Newline) => {}
                () if matches!(self.tok, Token::Eof) => break,

                // Return statements
                () if matches!(self.tok, Token::Return) => {
                    let return_stmt = self.parse_return_statement()?;
                    self.prog.statements.push(Box::new(return_stmt));
                }

                // Assign statements
                () if (matches!(self.tok, Token::Global)
                    || (matches!(self.tok, Token::Identifier(_)))
                        && matches!(self.peek_tok, Token::Equals)) =>
                {
                    let assign_stmt = self.parse_assign_statement()?;
                    self.prog.statements.push(Box::new(assign_stmt));
                }

                // Expression statements
                _ => {
                    let exp = self.parse_expr(Precedence::Lowest)?;
                    self.prog
                        .statements
                        .push(Box::new(ExpressionStatement { value: exp }));
                }
            }
            self.next_token()?;
        }
        Ok(())
    }

    fn parse_expr(&mut self, prec: Precedence) -> Result<Box<dyn Expression + 'a>, ParserError> {
        let mut left_expr = self.parse_left_expr()?;
        while !matches!(self.peek_tok, Token::Newline | Token::Eof) && prec < self.peek_tok.into() {
            self.next_token()?;
            left_expr = self.parse_infix_expression(left_expr)?;
        }
        Ok(left_expr)
    }

    fn parse_infix_expression(
        &mut self,
        left: Box<dyn Expression + 'a>,
    ) -> Result<Box<dyn Expression + 'a>, ParserError> {
        Ok(Box::new(InfixExpression {
            left,
            token: self.tok,
            operator: self.tok.try_into()?,
            right: {
                self.next_token()?;
                self.parse_expr(self.tok.into())?
            },
        }))
    }

    fn parse_left_expr(&mut self) -> Result<Box<dyn Expression + 'a>, ParserError> {
        use Token::*;

        match self.tok {
            Not | Plus | Minus => Ok(Box::new(self.parse_prefix_expr()?)),
            Identifier(_) => Ok(Box::new(self.parse_identifier()?)),
            NumberLiteral(_) => Ok(Box::new(self.parse_number_literal_expr()?)),
            True | False => Ok(Box::new(self.parse_bool_expr()?)),
            _ => Err(ParserError::UnexpectedToken),
        }
    }

    fn parse_prefix_expr(&mut self) -> Result<PrefixExpression<'a>, ParserError> {
        Ok(PrefixExpression {
            token:    self.tok,
            operator: match self.tok.try_into() {
                Ok(p) => p,
                Err(_) => return Err(ParserError::UnexpectedToken),
            },
            subject:  {
                self.next_token()?;
                self.parse_expr(Precedence::Prefix)?
            },
        })
    }

    fn parse_bool_expr(&mut self) -> Result<BooleanExpression<'a>, ParserError> {
        Ok(BooleanExpression {
            token: self.tok,
            value: match self.tok {
                Token::True => true,
                Token::False => false,
                _ => return Err(ParserError::UnexpectedToken),
            },
        })
    }

    fn parse_identifier(&mut self) -> Result<Identifier<'a>, ParserError> {
        if let Token::Identifier(_) = self.tok {
            Ok(Identifier { token: self.tok })
        } else {
            Err(ParserError::UnexpectedToken)
        }
    }

    fn parse_number_literal_expr(&mut self) -> Result<IntegerLiteralExpression<'a>, ParserError> {
        let token = self.tok;
        let value = match token {
            Token::NumberLiteral(n) => match n.parse() {
                Ok(i) => i,
                _ => return Err(ParserError::UnexpectedToken),
            },
            _ => return Err(ParserError::UnexpectedToken),
        };
        Ok(IntegerLiteralExpression { token, value })
    }

    fn parse_return_statement(&mut self) -> Result<ReturnStatement<'a>, ParserError> {
        let token = self.tok;
        self.next_token()?;
        match self.tok {
            Token::Newline | Token::Eof => Ok(ReturnStatement { token, value: None }),
            _ => {
                while !(matches!(self.tok, Token::Newline) || matches!(self.tok, Token::Eof)) {
                    self.next_token()?;
                }

                Ok(ReturnStatement {
                    token,
                    value: Some(Box::new(PlaceholderExpression {})),
                })
            }
        }
    }

    fn parse_assign_statement(&mut self) -> Result<AssignStatement<'a>, ParserError> {
        let token = self.tok;
        let ident;
        let mut global = false;
        match self.tok {
            Token::Global => {
                self.next_token()?;
                global = true;
                match self.tok {
                    Token::Identifier(_) => ident = self.tok,
                    _ => return Err(ParserError::UnexpectedToken),
                };
            }
            Token::Identifier(_) => ident = self.tok,
            _ => return Err(ParserError::UnexpectedToken),
        }
        self.next_token()?;

        if !matches!(self.tok, Token::Equals) {
            return Err(ParserError::UnexpectedToken);
        }

        while !(matches!(self.tok, Token::Newline) || matches!(self.tok, Token::Eof)) {
            self.next_token()?;
        }

        Ok(AssignStatement {
            token,
            global,
            ident: Identifier { token: ident },
            value: Box::new(PlaceholderExpression {}),
        })
    }

    pub fn next_token(&mut self) -> Result<(), LexerError> {
        self.tok = self.peek_tok;
        self.peek_tok = self.lexer.next_token()?;
        Ok(())
    }

    /// Might error in the rare case that the lexer is unable to continue in the
    /// first 2 tokens, for example an integer that's too big or an invalid
    /// string literal
    pub fn new(input: Lexer<'a>) -> Result<Self, LexerError> {
        let mut p = Self {
            lexer:    input,
            tok:      Token::default(),
            peek_tok: Token::default(),
            prog:     Program::default(),
        };
        // Read 2 tokens, so tok and read_tok are both set properly
        p.next_token()?;
        p.next_token()?;
        Ok(p)
    }
}

pub fn parse_from_lexer(input: Lexer) -> Result<Program, ParserError> {
    let mut parser = Parser::new(input).unwrap();
    parser.parse()?;
    Ok(std::mem::take(&mut parser.prog))
}

pub fn parse_from_string(input: &str) -> Result<Program, ParserError> {
    let mut parser = Parser::new(Lexer::new(input)).unwrap();
    parser.parse()?;
    Ok(std::mem::take(&mut parser.prog))
}
