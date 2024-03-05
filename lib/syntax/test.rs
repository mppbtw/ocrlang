use crate::lexer::Token;
use super::expressions::Identifier;
use super::ast::PrettyPrint;

#[test]
fn test_pretty_print_identifiers() {
    let ident = Identifier {
        token: Token::Identifier("test"),
    };
    assert_eq!(ident.pretty_print(), "test".to_owned());
}
