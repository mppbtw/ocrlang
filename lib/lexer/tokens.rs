/// The token emmitted by the lexer. It is worth noting that this is cheap to
/// copy and move around as it only contains references to data stored in the
/// input string.
#[derive(Hash, PartialEq, Eq, Debug, Default, Clone, Copy)]
pub enum Token<'a> {
    Identifier(&'a str),
    Equals,
    DoubleEquals,
    NotEqual,
    GThanOrEqual,
    LThanOrEqual,
    GThan,
    LThan,
    StringLiteral(&'a str),
    LParenthasis,
    RParenthasis,
    LSquirly,
    RSquirly,
    LSquareBracket,
    RSquareBracket,
    Plus,
    Asterisk,
    FSlash,
    Minus,
    Caret,
    Colon,

    /// This is just the literal string of the number, the parser will parse the
    /// number itself later; this is done to allow for cheap copying without
    /// the tokens actually holding any data just references to the input
    /// string
    NumberLiteral(&'a str),
    Comma,
    Eof,

    Global,
    For,
    Endfor,
    Next,
    While,
    Endwhile,
    Do,
    Until,
    And,
    If,
    Else,
    Or,
    Not,
    Endif,
    Return,
    Function,
    Endfunction,
    Then,
    Switch,
    Case,
    Default,
    Endswitch,
    Procedure,
    Endprocedure,
    Div,
    Mod,

    True,
    False,

    Newline,

    #[default]
    Illegal,
}

/// Check the identifier against a map of keywords, if none of them match then
/// Token::Identifier will be returned.
pub fn lookup_keyword(ident: &str) -> Token {
    use Token::*;

    match ident {
        "true" => True,
        "false" => False,
        "switch" => Switch,
        "endswitch" => Endswitch,
        "case" => Case,
        "default" => Default,
        "return" => Return,
        "for" => For,
        "endfor" => Endfor,
        "global" => Global,
        "do" => Do,
        "until" => Until,
        "if" => If,
        "else" => Else,
        "then" => Then,
        "OR" => Or,
        "NOT" => Not,
        "AND" => And,
        "DIV" => Div,
        "MOD" => Mod,
        "while" => While,
        "endwhile" => Endwhile,
        "next" => Next,
        "endif" => Endif,
        "procedure" => Procedure,
        "endprocedure" => Endprocedure,
        "function" => Function,
        "endfunction" => Endfunction,
        _ => Identifier(ident),
    }
}

impl Token<'_> {
    // Check if this can be used to end a block (BlockStatement), like endif/endfor etc.
    pub fn is_block_ender(&self) -> bool {
        use Token::*;
        matches!(self, Endif | Endfunction | Endprocedure | Endfor | Endwhile | Endswitch | Else)
    }
    /// Check if this token could be used as a prefix operator (+, -)
    pub fn is_prefix_op(&self) -> bool {
        matches!(self, Self::Plus | Self::Minus | Self::Not)
    }

    /// Check if this token could be used as a infix operator (+, -, DIV, MOD)
    pub fn is_infix_op(&self) -> bool {
        matches!(
            self,
            Self::Plus
                | Self::Minus
                | Self::Div
                | Self::Mod
                | Self::LThan
                | Self::LThanOrEqual
                | Self::GThan
                | Self::GThanOrEqual
                | Self::FSlash
                | Self::Or
                | Self::And
        )
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct TokenDebugInfo {
    pub tok_type: TokenType,
}
impl From<Token<'_>> for TokenDebugInfo {
    fn from(value: Token) -> Self {
        Self {
            tok_type: value.into(),
        }
    }
}

// Variation of the Token enum except with no attached data, only the variant
// type data is stored
#[derive(Hash, PartialEq, Eq, Debug, Default, Clone, Copy)]
pub enum TokenType {
    Identifier,
    Equals,
    DoubleEquals,
    NotEqual,
    GThanOrEqual,
    LThanOrEqual,
    GThan,
    LThan,
    StringLiteral,
    LParenthasis,
    RParenthasis,
    LSquirly,
    RSquirly,
    LSquareBracket,
    RSquareBracket,
    Plus,
    Asterisk,
    FSlash,
    Minus,
    Caret,
    Colon,
    NumberLiteral,
    Comma,
    Eof,

    Global,
    For,
    Endfor,
    Next,
    While,
    Endwhile,
    Do,
    Until,
    And,
    If,
    Else,
    Or,
    Not,
    Endif,
    Return,
    Function,
    Endfunction,
    Then,
    Switch,
    Case,
    Default,
    Endswitch,
    Procedure,
    Endprocedure,
    Div,
    Mod,

    True,
    False,

    Newline,

    #[default]
    Illegal,
}
impl From<Token<'_>> for TokenType {
    fn from(value: Token) -> Self {
        use TokenType::*;
        match value {
            Token::Identifier(_) => Identifier,
            Token::Equals => Equals,
            Token::DoubleEquals => DoubleEquals,
            Token::NotEqual => NotEqual,
            Token::GThanOrEqual => GThanOrEqual,
            Token::LThanOrEqual => LThanOrEqual,
            Token::GThan => GThan,
            Token::LThan => LThan,
            Token::StringLiteral(_) => StringLiteral,
            Token::LParenthasis => LParenthasis,
            Token::RParenthasis => RParenthasis,
            Token::LSquirly => LSquirly,
            Token::RSquirly => RSquirly,
            Token::LSquareBracket => LSquareBracket,
            Token::RSquareBracket => RSquareBracket,
            Token::Plus => Plus,
            Token::Asterisk => Asterisk,
            Token::FSlash => FSlash,
            Token::Minus => Minus,
            Token::Caret => Caret,
            Token::Colon => Colon,
            Token::NumberLiteral(_) => NumberLiteral,
            Token::Comma => Comma,
            Token::Eof => Eof,
            Token::Global => Global,
            Token::For => For,
            Token::Endfor => Endfor,
            Token::Next => Next,
            Token::While => While,
            Token::Endwhile => Endwhile,
            Token::Do => Do,
            Token::Until => Until,
            Token::And => And,
            Token::If => If,
            Token::Else => Else,
            Token::Or => Or,
            Token::Not => Not,
            Token::Endif => Endif,
            Token::Return => Return,
            Token::Function => Function,
            Token::Endfunction => Endfunction,
            Token::Then => Then,
            Token::Switch => Switch,
            Token::Case => Case,
            Token::Default => Default,
            Token::Endswitch => Endswitch,
            Token::Procedure => Procedure,
            Token::Endprocedure => Endprocedure,
            Token::Div => Div,
            Token::Mod => Mod,
            Token::True => True,
            Token::False => False,
            Token::Newline => Newline,
            Token::Illegal => Illegal,
        }
    }
}
