use super::Lexer;
use super::Token;

#[test]
fn test_tokenise_integers() {
        let input = "123   2";
    let expected = vec![
        Token::Number(123),
        Token::Number(2),
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
