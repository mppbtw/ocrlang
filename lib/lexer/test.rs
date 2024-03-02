use super::Lexer;
use super::Token;

#[test]
fn test_tokenise_integers() {
    let input = "123   2";
    let expected = vec![Token::Number(123), Token::Number(2), Token::Eof];
    let mut i = 0;
    let mut lexer = Lexer::new(input);

    loop {
        let expected_token = &expected[i];
        let tok = lexer.next_token().unwrap();
        assert_eq!(tok, *expected_token);
        if matches!(tok, Token::Eof) {
            break;
        }
        i += 1;
    }
}

#[test]
fn test_tokenise_brackets() {
    let input = "{}[]()";
    let expected = vec![
        Token::LSquirly,
        Token::RSquirly,
        Token::LSquareBracket,
        Token::RSquareBracket,
        Token::LParenthasis,
        Token::RParenthasis,
        Token::Eof,
    ];
    let mut i = 0;
    let mut lexer = Lexer::new(input);

    loop {
        let expected_token = &expected[i];
        let tok = lexer.next_token().unwrap();
        assert_eq!(tok, *expected_token);
        if matches!(tok, Token::Eof) {
            break;
        }
        i += 1;
    }
}

#[test]
fn test_tokenise_symbols() {
    let input = "+-,/: > == >= <= < !=";
    let expected = vec![
        Token::Plus,
        Token::Minus,
        Token::Comma,
        Token::FSlash,
        Token::Colon,
        Token::GThan,
        Token::DoubleEquals,
        Token::GThanOrEqual,
        Token::LThanOrEqual,
        Token::LThan,
        Token::NotEqual,
        Token::Eof
    ];
    let mut i = 0;
    let mut lexer = Lexer::new(input);

    loop {
        let expected_token = &expected[i];
        let tok = lexer.next_token().unwrap();
        assert_eq!(tok, *expected_token);
        if matches!(tok, Token::Eof) {
            break;
        }
        i += 1;
    }
}

#[test]
fn test_tokenise_keywords() {
    let input = "global for endfor next while endwhile do until AND if OR
        NOT endif return function endfunction then switch endswitch case default procedure endprocedure DIV MOD";
    let expected = vec![
        Token::Global,
        Token::For,
        Token::Endfor,
        Token::Next,
        Token::While,
        Token::Endwhile,
        Token::Do,
        Token::Until,
        Token::And,
        Token::If,
        Token::Or,
        Token::Not,
        Token::Endif,
        Token::Return,
        Token::Function,
        Token::Endfunction,
        Token::Then,
        Token::Switch,
        Token::Endswitch,
        Token::Case,
        Token::Default,
        Token::Procedure,
        Token::Endprocedure,
        Token::Div,
        Token::Mod,
        Token::Eof,
    ];

    let mut i = 0;
    let mut lexer = Lexer::new(input);
    loop {
        let expected_token = &expected[i];
        let tok = lexer.next_token().unwrap();
        assert_eq!(tok, *expected_token);
        if matches!(tok, Token::Eof) {
            break;
        }
        i += 1;
    }
}

#[test]
fn test_tokenise_ignore_comments() {
    let input = "
function // This is a comment
         // this is another line of comment
         global
endfunction";
    let expected = vec![
        Token::Function,
        Token::Global,
        Token::Endfunction,
        Token::Eof,
    ];

    let mut i = 0;
    let mut lexer = Lexer::new(input);
    loop {
        let expected_token = &expected[i];
        let tok = lexer.next_token().unwrap();
        assert_eq!(tok, *expected_token);
        if matches!(tok, Token::Eof) {
            break;
        }
        i += 1;
    }
}
