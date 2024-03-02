use super::ast::Program;
use crate::lexer::Lexer;
use crate::lexer::LexerError;
use crate::lexer::Token;

pub struct Parser<'a> {
    lexer:    Lexer<'a>,
    tok:      Token<'a>,
    read_tok: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn parse(&self) -> Program {
        Program::default()
    }

    pub fn next_token(&mut self) -> Result<(), LexerError> {
        self.tok = self.read_tok;
        self.read_tok = self.lexer.next_token()?;
        Ok(())
    }
}
