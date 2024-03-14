use super::parser::parse_from_lexer;
use crate::lexer::Lexer;
use crate::syntax::{Statement, StatementType};

#[test]
fn test_parse_var_assign_statement() {
    let input = "a = 1
        global bb = 22
        ccc = 333";
    let prog = parse_from_lexer(Lexer::new(input)).unwrap();
    assert_eq!(prog.statements.len(), 3);

    assert!(matches!(prog.statements[0].get_type(), StatementType::Assign(_)));
    if let StatementType::Assign(stmt) = prog.statements[0].get_type() {
        assert!(!stmt.global);
        assert_eq!(stmt.ident.get_ident(), "a");
    }

    assert!(matches!(prog.statements[1].get_type(), StatementType::Assign(_)));
    if let StatementType::Assign(stmt) = prog.statements[1].get_type() {
        assert_eq!(stmt.ident.get_ident(), "bb");
        assert!(stmt.global);
    }

    assert!(matches!(prog.statements[2].get_type(), StatementType::Assign(_)));
    if let StatementType::Assign(stmt) = prog.statements[2].get_type() {
        assert_eq!(stmt.ident.get_ident(), "ccc");
        assert!(!stmt.global);
    }
}

#[test]
fn test_parse_return_statement() {}

#[test]
fn test_parse_integer_literal_expression() {}
