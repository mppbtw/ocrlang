use crate::lexer::Token;
use std::default::Default;

#[derive(Default, Debug)]
pub struct Lexer<'a> {
    pos: usize,
    read_pos: usize,
    input: &'a str,
    ch: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut l = Self {
            input,
            ..Default::default()
        };
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Token {
        Token::Eof
    }

    fn read_char(&mut self) {}
}
