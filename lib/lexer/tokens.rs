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
        _ => Identifier(ident)
    }
}

impl Token<'_> {
    // Check if this token could be used as a prefix operator (+, -)
    pub fn is_prefix_op(&self) -> bool {
        matches!(self, Self::Plus | Self::Minus | Self::Not)
    }

    // Check if this token could be used as a infix operator (+, -, DIV, MOD)
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
