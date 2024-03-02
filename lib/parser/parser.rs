use std::cell::RefCell;

use super::ast::Program;
use super::ast::Statement;
use crate::lexer::Lexer;
use crate::lexer::LexerError;
use crate::lexer::Token;

pub enum ParserError {
    InvalidNumberLiteral,
    TooLargeInteger,
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
    read_tok: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn parse(&mut self) -> Result<Program, ParserError> {
        let mut prog = Program::default();
        while matches!(self.tok, Token::Eof) {
            if matches!(self.tok, Token::Identifier(_)) {
                prog.statements.push(self.parse_assign_statement()?);
            }
            self.next_token()?;
        }
        Ok(prog)
    }

    fn parse_assign_statement(&mut self) -> Result<Statement<'a>, ParserError> {
        Ok(Statement::default())
    }

    pub fn next_token(&mut self) -> Result<(), LexerError> {
        self.tok = self.read_tok;
        self.read_tok = self.lexer.next_token()?;
        Ok(())
    }

    /// Might error in the rare case that the lexer is unable to continue in the
    /// first 2 tokens, for example an integer that's too big or an invalid
    /// string literal
    pub fn new(input: Lexer<'a>) -> Result<Self, LexerError> {
        let mut p = Self {
            lexer:    input,
            tok:      Token::default(),
            read_tok: Token::default(),
        };
        // Read 2 tokens, so tok and read_tok are both set properly
        p.next_token()?;
        p.next_token()?;
        Ok(p)
    }
}
