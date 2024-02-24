use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Hash, PartialEq, Eq, Debug, Default, Clone, Copy)]
pub enum Token<'a> {
    Identifier(&'a str),
    Equals,
    DoubleEquals,
    NotEqual,
    GThanOrequals,
    LThanOrequals,
    GThan,
    LThan,
    Global,
    StringLiteral(&'a str),
    LeftBracket,
    RightBracket,
    LeftSquirly,
    RightSquirly,
    LeftSquareBracket,
    RightSquareBracket,
    For,
    Next,
    While,
    Endwhile,
    Do,
    Until,
    And,
    If,
    Or,
    Not,
    Plus,
    Asterisk,
    FSlash,
    BSlash,
    Minus,
    Then,
    Switch,
    Case,
    Default,
    Endswitch,
    Procedure,
    Endprocedure,
    Div,
    Mod,
    Carat,
    Colon,
    Number(i128),
    Comma,
    Endif,
    Return,
    Function,
    Endfunction,

    #[default]
    Eof,
}

/// Map of the keyword's literal appearance in code to the tokens of that
/// literal, e.g. 'return' => Token::Return.
static KEYWORDS: LazyLock<HashMap<&str, Token>> = LazyLock::new(|| {
    use Token::*;

    HashMap::from([
        ("return", Return),
        ("for", For),
        ("global", Global),
        ("do", Do),
        ("until", Until),
        ("if", If),
        ("OR", Or),
        ("NOT", Not),
        ("AND", And),
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

/// Fancy names of tokens used for debug/error prints; this does not give you
/// any data stored in the token like the value of an integer or a string. The
/// formatting goes like this:
/// * Keywords are the same as they appear in code, Token::For => 'for'
/// * Operators like `>`, `==` are given literal names in UpperCamelCase:
///   'GreaterThan', 'DoubleEquals'
/// * Special characters are the same: Token::Colon => 'Colon'
static PRETTY_TOKEN_NAMES: LazyLock<HashMap<Token, &str>> = LazyLock::new(|| {
    use Token::*;
    HashMap::from([(While, "while"), (For, "for")])
});
