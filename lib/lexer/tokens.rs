use std::collections::HashMap;
use std::sync::LazyLock;

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

    Newline,

    #[default]
    Illegal,
}

/// Map of the keyword's literal appearance in code to the tokens of that
/// literal, e.g. 'return' => Token::Return.
static KEYWORDS: LazyLock<HashMap<&str, Token>> = LazyLock::new(|| {
    use Token::*;

    HashMap::from([
        ("then", Then),
        ("switch", Switch),
        ("endswitch", Endswitch),
        ("case", Case),
        ("default", Default),
        ("return", Return),
        ("for", For),
        ("endfor", Endfor),
        ("global", Global),
        ("do", Do),
        ("until", Until),
        ("if", If),
        ("OR", Or),
        ("NOT", Not),
        ("AND", And),
        ("DIV", Div),
        ("MOD", Mod),
        ("while", While),
        ("endwhile", Endwhile),
        ("next", Next),
        ("endif", Endif),
        ("procedure", Procedure),
        ("endprocedure", Endprocedure),
        ("function", Function),
        ("endfunction", Endfunction),
    ])
});

/// Check the identifier against a map of keywords, if none of them match then
/// Token::Identifier will be returned.
pub fn lookup_keyword(ident: &str) -> Token {
    // I don't like using `get_key_value` here but apparently `LazyLock<HashMap<_,
    // _>>` doesn't have a regular get method for just the value.
    match KEYWORDS.get_key_value(ident) {
        Some((_, b)) => b.to_owned(),
        None => Token::Identifier(ident),
    }
}

static PRETTY_TOKEN_NAMES: LazyLock<HashMap<Token, &str>> = LazyLock::new(|| {
    use Token::*;
    HashMap::from([
        (While, "while"),
        (Endwhile, "endwhile"),
        (For, "for"),
        (Endfor, "endfor"),
        (If, "id"),
        (Global, "global"),
        (Next, "next"),
        (Do, "do"),
        (Until, "until"),
        (Or, "or"),
        (Not, "not"),
        (And, "AND"),
        (Endif, "endif"),
        (Return, "return"),
        (Function, "function"),
        (Endfunction, "endfunction"),
        (Then, "then"),
        (Switch, "switch"),
        (Case, "case"),
        (Default, "default"),
        (Endswitch, "endswitch"),
        (Procedure, "procedure"),
        (Endprocedure, "endprocedure"),
        (Equals, "Equals"),
        (DoubleEquals, "DoubleEquals"),
        (NotEqual, "NotEqual"),
        (GThanOrEqual, "GreaterThanOrEqual"),
        (LThanOrEqual, "LessThanOrEqual"),
        (GThan, "GreaterThan"),
        (LThan, "LessThan"),
        (LParenthasis, "LeftParenthasis"),
        (RParenthasis, "RightParenthasis"),
        (LSquirly, "RightSquirlyBrace"),
        (RSquirly, "DoubleEquals"),
        (LSquareBracket, "LeftSquareBracket"),
        (RSquareBracket, "RightSquareBracket"),
        (Plus, "Plus"),
        (Asterisk, "Asterisk"),
        (FSlash, "ForwardSlash"),
        (Minus, "Minus"),
        (Div, "DIV"),
        (Mod, "MOD"),
        (Caret, "Caret"),
        (Colon, "Colon"),
        (Comma, "Comma"),
        (Eof, "EndOfFile"),
        (Newline, "Newline"),
        (Illegal, "Illegal"),
    ])
});

impl Token<'_> {
    /// Fancy names of tokens used for debug/error prints.
    pub fn pretty_name(&self) -> String {
        // The hashmap only works for tokens that don't hold data as that requires
        // expression matching, there might be a better solution but for now they
        // are checked in this abstraction
        (match self {
            Token::Identifier(_) => "Identifier",
            Token::StringLiteral(_) => "StringLiteral",
            _ => PRETTY_TOKEN_NAMES.get_key_value(self).unwrap().1,
        })
        .to_owned()
    }
}
