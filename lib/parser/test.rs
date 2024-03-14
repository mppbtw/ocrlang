use super::Parser;
use crate::lexer::Lexer;
use crate::syntax::{Statement, StatementType};

#[test]
fn test_parse_var_assign_statement() {
    let input = "a = 1
        global bb = 22
        ccc = 333";
    let mut parser = Parser::new(Lexer::new(input)).unwrap();
    let mut prog = parser.parse().unwrap();
}

#[test]
fn test_parse_return_statement() {}

#[test]
fn test_parse_integer_literal_expression() {}
