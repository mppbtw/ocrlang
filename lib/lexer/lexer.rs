use crate::lexer::Token;

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

    pub fn next_token(&mut self) -> Token {
        Token::Eof
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

    fn skip_to_next_line(&mut self) {
        while self.ch != b'\n' && self.ch != 0 {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.pos;
        while self.ch.is_ascii_alphanumeric() || self.ch == b'_' {
            self.read_char();
        }

        // This needs to subtract because when calling `read_char` at the last character of the
        // identier, the `read_pos` advances to 2 places beyond the identifier; this is fatal if
        // the next character isn't whitespace and it messes up the column/line count
        self.read_pos -= 1;
        self.input[pos..self.pos].to_owned()
    }
}
