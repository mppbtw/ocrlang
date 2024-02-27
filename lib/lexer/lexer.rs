use crate::lexer::Token;

use super::tokens::lookup_keyword;

#[derive(Debug, Clone)]
pub enum LexerError {}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Lexer<'a> {
    pos:      usize,
    read_pos: usize,
    input:    &'a str,
    ch:       u8,
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

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        self.munch_whitespace();

        // Deal with comments
        loop {
            if self.ch == b'/' && self.peek_char() == b'/' {
                self.skip_to_next_line();
            } else {
                break;
            }
        }

        let tok: Token = match self.ch {
            b'\\' => Token::BSlash,
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'(' => Token::LParenthasis,
            b')' => Token::RParenthasis,
            b'[' => Token::LSquareBracket,
            b']' => Token::RSquareBracket,
            b',' => Token::Comma,
            b'/' => Token::FSlash,
            b':' => Token::Colon,
            b'{' => Token::LSquirly,
            b'}' => Token::RSquirly,
            b'>' => {
                if self.peek_char() == b'=' {
                    Token::GThanOrEqual
                } else {
                    Token::GThan
                }
            }
            b'<' => {
                if self.peek_char() == b'=' {
                    Token::LThanOrEqual
                } else {
                    Token::LThan
                }
            }
            b'=' => {
                if self.peek_char() == b'=' {
                    Token::DoubleEquals
                } else {
                    Token::Equals
                }
            }
            b'!' => {
                if self.peek_char() == b'=' {
                    Token::NotEqual
                } else {
                    Token::Illegal
                }
            }
            _ => {
                if self.ch.is_ascii_alphanumeric() {
                    lookup_keyword(self.read_identifier())
                } else {
                    Token::Illegal
                }
            }
        };
        self.read_char();
        Ok(tok)
    }

    fn peek_char(&self) -> u8 {
        if self.read_pos >= self.input.len() {
            0
        } else {
            self.input.bytes().collect::<Vec<u8>>()[self.read_pos]
        }
    }

    fn read_char(&mut self) {
        if self.read_pos >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_pos];
        }
        self.pos = self.read_pos;
        self.read_pos += 1;
    }

    fn munch_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }

        // This needs to subtract because when calling `read_char` at the last character
        // of the identier, the `read_pos` advances to 2 places beyond the
        // identifier; this is fatal if the next character isn't whitespace and
        // it messes up the column/line count
        self.read_pos -= 1;
    }

    fn skip_to_next_line(&mut self) {
        while self.ch != b'\n' && self.ch != 0 {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> &'a str {
        let pos = self.pos;
        while self.ch.is_ascii_alphanumeric() || self.ch == b'_' {
            self.read_char();
        }

        // This needs to subtract because when calling `read_char` at the last character
        // of the identier, the `read_pos` advances to 2 places beyond the
        // identifier; this is fatal if the next character isn't whitespace and
        // it messes up the column/line count
        self.read_pos -= 1;
        &self.input[pos..self.pos]
    }
}
