use super::ast::Program;
use crate::lexer::Lexer;
use crate::lexer::LexerError;
use crate::lexer::Token;

#[derive(Default, Debug)]
pub struct Parser<'a> {
    lexer:    Lexer<'a>,
    tok:      Token<'a>,
    read_tok: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn parse(&self) -> Result<Program, ()> {
        Ok(Program::default())
    }

    pub fn next_token(&mut self) -> Result<(), LexerError> {
        self.tok = self.read_tok;
        self.read_tok = self.lexer.next_token()?;
        Ok(())
    }

    /// Might error in the rare case that the lexer is unable to continue in the
    /// first 2 tokens, for example an integer that's too big or invalid
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
