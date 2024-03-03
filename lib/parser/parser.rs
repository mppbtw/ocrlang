use super::ast::Identifier;
use super::ast::Program;
use super::ast::Statement;
use crate::lexer::Lexer;
use crate::lexer::LexerError;
use crate::lexer::Token;

#[derive(Debug, Clone, Copy, Default)]
pub enum ParserError {
    InvalidNumberLiteral,
    TooLargeInteger,

    #[default]
    UnexpectedToken,
}
impl From<LexerError> for ParserError {
    fn from(value: LexerError) -> Self {
        match value {
            LexerError::TooLargeInteger => ParserError::TooLargeInteger,
            LexerError::InvalidNumberLiteral => ParserError::InvalidNumberLiteral,
        }
    }
}

#[derive(Default, Debug)]
pub struct Parser<'a> {
    lexer:    Lexer<'a>,
    tok:      Token<'a>,
    peek_tok: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn parse(&mut self) -> Result<Program, ParserError> {
        let mut prog = Program::default();
        while !matches!(self.tok, Token::Eof) {
            // Variable assign statements
            if matches!(self.tok, Token::Global)
                || (matches!(self.tok, Token::Identifier(_))
                    && matches!(self.peek_tok, Token::Equals))
            {
                prog.statements.push(self.parse_assign_statement()?);
            } else if matches!(self.tok, Token::Return) {
                prog.statements.push(self.parse_return_statement()?);
            }
            self.next_token()?;
        }
        Ok(prog)
    }

    fn parse_return_statement(&mut self) -> Result<Statement<'a>, ParserError> {
        let token = self.tok.clone();
        while !(matches!(self.tok, Token::Newline) || matches!(self.tok, Token::Eof)) {
            self.next_token()?;
        }

        Ok(Statement::Return { token, value: None })
    }

    fn parse_assign_statement(&mut self) -> Result<Statement<'a>, ParserError> {
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

        Ok(Statement::Assign {
            token,
            global,
            ident: Identifier::new(ident),
            value: None,
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
        };
        // Read 2 tokens, so tok and read_tok are both set properly
        p.next_token()?;
        p.next_token()?;
        Ok(p)
    }
}
