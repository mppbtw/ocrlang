use super::parser::parse_from_lexer;
use crate::lexer::Lexer;
use crate::syntax::Statement;

#[test]
fn test_parse_var_assign_statement() {
    let input = "a = 1
        global bb = 22
        ccc = 333";
    let prog = parse_from_lexer(Lexer::new(input)).unwrap();
    assert_eq!(prog.statements.len(), 3);
}

#[test]
fn test_parse_return_statement() {}

#[test]
fn test_parse_integer_literal_expression() {}
