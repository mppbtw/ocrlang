use crate::lexer::Lexer;
use crate::lexer::LexerError;
use crate::lexer::Token;
use crate::lexer::TokenDebugInfo;
use crate::syntax::AssignStatement;
use crate::syntax::BlockStatement;
use crate::syntax::BooleanExpression;
use crate::syntax::Expression;
use crate::syntax::ExpressionStatement;
use crate::syntax::ExpressionType;
use crate::syntax::FunctionCallExpression;
use crate::syntax::FunctionStatement;
use crate::syntax::Identifier;
use crate::syntax::IfStatement;
use crate::syntax::InfixExpression;
use crate::syntax::IntegerLiteralExpression;
use crate::syntax::NoSuchInfixOperatorError;
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
            Plus | Minus => Self::Sum,
            FSlash | Asterisk | Mod | Div => Self::Product,
            And => Self::And,
            Or => Self::Or,
            LParenthasis => Self::Call,
            _ => Self::Lowest,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ParserError {
    UnterminatedStringLiteral,
    InvalidNumberLiteral,
    TooLargeInteger,

    UnexpectedToken(TokenDebugInfo),
}
impl From<LexerError> for ParserError {
    fn from(value: LexerError) -> Self {
        match value {
            LexerError::UnterminatedStringLiteral => Self::UnterminatedStringLiteral,
        }
    }
}
impl From<NoSuchInfixOperatorError<'_>> for ParserError {
    fn from(value: NoSuchInfixOperatorError) -> ParserError {
        ParserError::UnexpectedToken(value.tok.into())
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
            match self.parse_statement()? {
                Some(s) => self.prog.statements.push(s),
                None => break,
            }
            self.next_token()?;
        }
        Ok(())
    }

    /// Returns Ok(None) only in the case of Token::Eof
    fn parse_statement(&mut self) -> Result<Option<Box<dyn Statement + 'a>>, ParserError> {
        loop {
            match () {
                () if matches!(self.tok, Token::Newline) => (),
                () if matches!(self.tok, Token::Eof) => return Ok(None),

                // Return statements
                () if matches!(self.tok, Token::Return) => {
                    let return_stmt = self.parse_return_statement()?;
                    return Ok(Some(Box::new(return_stmt)));
                }

                // If statements
                () if matches!(self.tok, Token::If) => {
                    let if_stmt = self.parse_if_statement()?;
                    return Ok(Some(Box::new(if_stmt)));
                }

                // Function/procedure declaration
                () if matches!(self.tok, Token::Function)
                    || matches!(self.tok, Token::Procedure) =>
                {
                    let func = self.parse_function()?;
                    return Ok(Some(Box::new(func)));
                }

                // Assign statements
                () if (matches!(self.tok, Token::Global)
                    || (matches!(self.tok, Token::Identifier(_)))
                        && matches!(self.peek_tok, Token::Equals)) =>
                {
                    dbg!("parsing assign at", self.tok);
                    let assign_stmt = self.parse_assign_statement()?;
                    dbg!(&assign_stmt);
                    return Ok(Some(Box::new(assign_stmt)));
                }

                // Expression statements
                _ => {
                    let exp = self.parse_expr(Precedence::Lowest)?;
                    return Ok(Some(Box::new(ExpressionStatement { value: exp })));
                }
            }
            self.next_token()?;
        }
    }

    fn parse_expr(&mut self, prec: Precedence) -> Result<Box<dyn Expression + 'a>, ParserError> {
        let ident = match self.tok {
            Token::Identifier(_) => Some(self.tok.into()),
            _ => None,
        };
        let mut left_expr = self.parse_left_expr()?;
        while !matches!(self.peek_tok, Token::Newline | Token::Eof) && prec < self.peek_tok.into() {
            self.next_token()?;
            left_expr = self.parse_infix_expression(left_expr, ident.clone())?;
        }
        Ok(left_expr)
    }

    fn parse_infix_expression(
        &mut self,
        left: Box<dyn Expression + 'a>,
        ident: Option<Identifier<'a>>,
    ) -> Result<Box<dyn Expression + 'a>, ParserError> {
        if self.tok == Token::LParenthasis {
            match ident {
                Some(i) => {
                    Ok(Box::new(self.parse_function_call(i)?))
                }
                None => Err(ParserError::UnexpectedToken(self.tok.into())),
            }
        } else {
            Ok(Box::new(InfixExpression {
                left,
                token: self.tok,
                operator: self.tok.try_into()?,
                right: {
                    let prec: Precedence = self.tok.into();
                    self.next_token()?;
                    self.parse_expr(prec)?
                }
            },
            ))
    }
    }

    fn parse_left_expr(&mut self) -> Result<Box<dyn Expression + 'a>, ParserError> {
        use Token::*;

        match self.tok {
            Not | Plus | Minus => Ok(Box::new(self.parse_prefix_expr()?)),
            Identifier(_) => Ok(Box::new(self.parse_identifier()?)),
            NumberLiteral(_) => Ok(Box::new(self.parse_number_literal_expr()?)),
            True | False => Ok(Box::new(self.parse_bool_expr()?)),
            LParenthasis => Ok(self.parse_grouped_expr()?),
            _ => Err(ParserError::UnexpectedToken(self.tok.into())),
        }
    }

    fn parse_grouped_expr(&mut self) -> Result<Box<dyn Expression + 'a>, ParserError> {
        self.next_token()?;
        let expr = self.parse_expr(Precedence::Lowest)?;
        self.next_token()?;
        match self.tok {
            Token::RParenthasis => Ok(expr),
            _ => Err(ParserError::UnexpectedToken(self.tok.into())),
        }
    }

    fn parse_function_call(
        &mut self,
        identifier: Identifier<'a>,
    ) -> Result<FunctionCallExpression<'a>, ParserError> {
        Ok(FunctionCallExpression {
            token: identifier.token,
            func:  identifier.token.into(),
            args:  self.parse_call_args()?,
        })
    }

    fn parse_call_args(&mut self) -> Result<Vec<Box<dyn Expression + 'a>>, ParserError> {
        let mut args = Vec::new();
        // The current token should be the LParenthasis
        self.next_token()?;
        self.skip_newlines()?;
        if self.tok == Token::RParenthasis {
            return Ok(args);
        }
        args.push(self.parse_expr(Precedence::Lowest)?);
        self.next_token()?;
        self.skip_newlines()?;
        while self.tok == Token::Comma {
            self.next_token()?;
            self.skip_newlines()?;
            args.push(self.parse_expr(Precedence::Lowest)?);
            self.next_token()?;
        }
        if self.tok != Token::RParenthasis {
            return Err(ParserError::UnexpectedToken(self.tok.into()));
        }

        Ok(args)
    }

    fn parse_prefix_expr(&mut self) -> Result<PrefixExpression<'a>, ParserError> {
        Ok(PrefixExpression {
            token:    self.tok,
            operator: match self.tok.try_into() {
                Ok(p) => p,
                Err(_) => return Err(ParserError::UnexpectedToken(self.tok.into())),
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
                _ => return Err(ParserError::UnexpectedToken(self.tok.into())),
            },
        })
    }

    fn parse_identifier(&mut self) -> Result<Identifier<'a>, ParserError> {
        if let Token::Identifier(_) = self.tok {
            Ok(Identifier { token: self.tok })
        } else {
            Err(ParserError::UnexpectedToken(self.tok.into()))
        }
    }

    fn parse_number_literal_expr(&mut self) -> Result<IntegerLiteralExpression<'a>, ParserError> {
        let token = self.tok;
        let value = match token {
            Token::NumberLiteral(n) => match n.parse() {
                Ok(i) => i,
                _ => return Err(ParserError::UnexpectedToken(self.tok.into())),
            },
            _ => return Err(ParserError::UnexpectedToken(self.tok.into())),
        };
        Ok(IntegerLiteralExpression { token, value })
    }

    fn parse_function(&mut self) -> Result<FunctionStatement<'a>, ParserError> {
        let token = self.tok;
        let is_procedure = matches!(self.tok, Token::Procedure);
        self.next_token()?;
        let ident = match self.tok {
            Token::Identifier(_) => self.tok.into(),
            _ => return Err(ParserError::UnexpectedToken(self.tok.into())),
        };

        self.next_token()?;
        if !matches!(self.tok, Token::LParenthasis) {
            return Err(ParserError::UnexpectedToken(self.tok.into()));
        }

        let mut params = Vec::new();
        loop {
            self.next_token()?;
            match self.tok {
                Token::Identifier(_) => params.push(self.tok.into()),
                Token::RParenthasis => break,
                _ => return Err(ParserError::UnexpectedToken(self.tok.into())),
            };

            self.next_token()?;
            match self.tok {
                Token::Comma => (),
                Token::RParenthasis => break,
                _ => return Err(ParserError::UnexpectedToken(self.tok.into())),
            };
        }
        self.next_token()?; // Skip past the RParenthasis
        self.skip_newlines()?;
        let body = self.parse_block_statement()?;
        match self.tok {
            Token::Endfunction => {
                if is_procedure {
                    return Err(ParserError::UnexpectedToken(self.tok.into()));
                }
            }
            Token::Endprocedure => {
                if !is_procedure {
                    return Err(ParserError::UnexpectedToken(self.tok.into()));
                }
            }
            _ => return Err(ParserError::UnexpectedToken(self.tok.into())),
        }
        self.skip_newlines()?;

        Ok(FunctionStatement {
            token,
            is_procedure,
            body,
            ident,
            params,
        })
    }

    fn parse_if_statement(&mut self) -> Result<IfStatement<'a>, ParserError> {
        // If <expr> then
        //    <block>
        // (else
        //    <block>)
        // endif
        let token = self.tok;
        self.next_token()?;
        let condition = self.parse_expr(Precedence::Lowest)?;

        self.next_token()?;
        if !matches!(self.tok, Token::Then) {
            return Err(ParserError::UnexpectedToken(self.tok.into()));
        }
        self.next_token()?;

        let consequence = self.parse_block_statement()?;

        Ok(IfStatement {
            token,
            condition,
            consequence,
            alternative: {
                if let Token::Else = self.tok {
                    self.next_token()?;
                    let block = self.parse_block_statement()?;
                    if !matches!(self.tok, Token::Endif) {
                        return Err(ParserError::UnexpectedToken(self.tok.into()));
                    }
                    Some(block)
                } else {
                    None
                }
            },
        })
    }

    fn parse_block_statement(&mut self) -> Result<BlockStatement<'a>, ParserError> {
        let mut block = BlockStatement {
            token:      self.tok,
            statements: vec![],
        };
        while !(self.tok.is_block_ender() || matches!(self.tok, Token::Eof)) {
            if let Some(s) = self.parse_statement()? {
                block.statements.push(s);
            }

            self.skip_newlines()?;
            self.next_token()?;
            self.skip_newlines()?;
        }
        Ok(block)
    }

    fn parse_return_statement(&mut self) -> Result<ReturnStatement<'a>, ParserError> {
        let token = self.tok;
        match self.peek_tok {
            Token::Newline | Token::Eof => Ok(ReturnStatement { token, value: None }),
            _ => Ok(ReturnStatement {
                token,
                value: {
                    let prec: Precedence = self.tok.into();
                    match self.peek_tok {
                        Token::Newline | Token::Eof => None,
                        _ => {
                            self.next_token()?;
                            Some(self.parse_expr(prec)?)
                        }
                    }
                },
            }),
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
                    _ => return Err(ParserError::UnexpectedToken(self.tok.into())),
                };
            }
            Token::Identifier(_) => ident = self.tok,
            _ => return Err(ParserError::UnexpectedToken(self.tok.into())),
        }
        self.next_token()?;

        if !matches!(self.tok, Token::Equals) {
            return Err(ParserError::UnexpectedToken(self.tok.into()));
        }

        Ok(AssignStatement {
            token,
            global,
            ident: Identifier { token: ident },
            value: {
                let prec: Precedence = self.tok.into();
                self.next_token()?;
                self.parse_expr(prec)?
            },
        })
    }

    fn skip_newlines(&mut self) -> Result<(), ParserError> {
        while matches!(self.tok, Token::Newline) {
            self.next_token()?;
        }
        Ok(())
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
