use super::ast::Identifier;
use super::ast::PrettyPrint;
use super::PlaceholderExpression;
use super::PrefixExpression;
use crate::lexer::Token;
use crate::syntax::InfixExpression;
use crate::syntax::InfixOperator;
use crate::syntax::PrefixOperator;

#[test]
fn test_pretty_print_identifiers() {
    let ident = Identifier {
        token: Token::Identifier("ItsGoodToBeD"),
    };
    assert_eq!(ident.pretty_print(), "ItsGoodToBeD".to_owned());
}

#[test]
fn test_pretty_print_prefix_op() {
    let op = PrefixExpression {
        subject: Box::new(PlaceholderExpression{}),
        operator: PrefixOperator::Minus,
        token: Token::default(),
    };
    assert_eq!(op.pretty_print(), "-<PLACEHOLDER_EXPRESSION>");

    let op = PrefixExpression {
        subject: Box::new(PlaceholderExpression{}),
        operator: PrefixOperator::Not,
        token: Token::default(),
    };
    assert_eq!(op.pretty_print(), "NOT <PLACEHOLDER_EXPRESSION>")
}

#[test]
fn test_pretty_print_infix_op() {
    let op = InfixExpression {
        operator: InfixOperator::Divide,
        left: Box::new(PlaceholderExpression{}),
        right: Box::new(PlaceholderExpression{}),
        token: Token::default(),
    };
    assert_eq!(op.pretty_print(), "<PLACEHOLDER_EXPRESSION>/<PLACEHOLDER_EXPRESSION>")
}
