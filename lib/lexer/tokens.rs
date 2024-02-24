use std::sync::LazyLock;
use std::collections::HashMap;

/// Map of the keyword's literal appearance in code to the tokens of that literal,
/// e.g. 'return' => Token::Return
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

#[derive(Hash, PartialEq, Eq, Debug, Default)]
pub enum Token {
    Identifier,
    Equal,
    DoubleEqual,
    NotEqual,
    GThanOrEqual,
    LThanOrEqual,
    GThan,
    LThan,
    Global,
    StringLiteral(String),
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
    Eof
}
