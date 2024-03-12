use crate::lexer::Lexer;
use crate::lexer::LexerError;
use crate::lexer::Token;
use crate::syntax::Identifier;
use crate::syntax::IntegerLiteralExpression;
use crate::syntax::PlaceholderExpression;
use crate::syntax::Program;
use crate::syntax::ReturnStatement;
use crate::syntax::AssignStatement;
use crate::syntax::Statement;

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

#[derive(Default, Debug)]
pub struct Parser<'a> {
    lexer:    Lexer<'a>,
    tok:      Token<'a>,
    peek_tok: Token<'a>,
    prog:     Program<'a>,
}

impl<'a> Parser<'a> {
    pub fn parse(&mut self) -> Result<&'a Program, ParserError> {
        while !matches!(self.tok, Token::Eof) {
            // Variable assign statements
            if matches!(self.tok, Token::Global)
                || (matches!(self.tok, Token::Identifier(_))
                    && matches!(self.peek_tok, Token::Equals))
            {
                let assign_stmt = self.parse_assign_statement()?;
                self.prog.statements.push(Box::new(assign_stmt));
            } else if matches!(self.tok, Token::Return) {
                let return_stmt = self.parse_return_statement()?;
                self.prog.statements.push(Box::new(return_stmt));
            }
            self.next_token()?;
        }
        Ok(&self.prog)
    }

    fn parse_return_statement(&mut self) -> Result<ReturnStatement<'a>, ParserError> {
        let token = self.tok.clone();
        while !(matches!(self.tok, Token::Newline) || matches!(self.tok, Token::Eof)) {
            self.next_token()?;
        }

        Ok(ReturnStatement { token, value: None })
    }

    fn parse_assign_statement(&mut self) -> Result<AssignStatement<'a>, ParserError> {
        let token = self.tok.clone();
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
            value: Box::new(PlaceholderExpression{}),
        })
    }

    fn parse_integer_literal_expression(
        &mut self,
    ) -> Result<IntegerLiteralExpression, ParserError> {
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
            prog: Program::default(),
        };
        // Read 2 tokens, so tok and read_tok are both set properly
        p.next_token()?;
        p.next_token()?;
        Ok(p)
    }
}
