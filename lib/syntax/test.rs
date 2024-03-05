use crate::lexer::Token;
use super::ast::Identifier;
use super::ast::PrettyPrint;

#[test]
fn test_pretty_print_identifiers() {
    let ident = Identifier {
        token: Token::Identifier("ItsGoodToBeD"),
    };
    assert_eq!(ident.pretty_print(), "ItsGoodToBeD".to_owned());
}
