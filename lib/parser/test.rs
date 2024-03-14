use super::parse_from_string;
use crate::syntax::StatementType;

#[test]
fn test_parse_var_assign_statement() {
    let input = "a = 1
        global bb = 22
        ccc = 333";
    let prog = parse_from_string(input).unwrap();
    assert_eq!(prog.statements.len(), 3);

    assert!(matches!(
        prog.statements[0].get_type(),
        StatementType::Assign(_)
    ));
    if let StatementType::Assign(stmt) = prog.statements[0].get_type() {
        assert!(!stmt.global);
        assert_eq!(stmt.ident.get_ident(), "a");
    }

    assert!(matches!(
        prog.statements[1].get_type(),
        StatementType::Assign(_)
    ));
    if let StatementType::Assign(stmt) = prog.statements[1].get_type() {
        assert_eq!(stmt.ident.get_ident(), "bb");
        assert!(stmt.global);
    }

    assert!(matches!(
        prog.statements[2].get_type(),
        StatementType::Assign(_)
    ));
    if let StatementType::Assign(stmt) = prog.statements[2].get_type() {
        assert_eq!(stmt.ident.get_ident(), "ccc");
        assert!(!stmt.global);
    }
}

#[test]
fn test_parse_return_statement() {
    let input = "return abc
        return";
    let prog = parse_from_string(input).unwrap();
    assert_eq!(prog.statements.len(), 2);

    assert!(matches!(
        prog.statements[0].get_type(),
        StatementType::Return(_)
    ));
    if let StatementType::Return(stmt) = prog.statements[0].get_type() {
        assert!(stmt.value.is_some())
    }

    assert!(matches!(
        prog.statements[1].get_type(),
        StatementType::Return(_)
    ));
    if let StatementType::Return(stmt) = prog.statements[1].get_type() {
        assert!(stmt.value.is_none())
    }
}

#[test]
fn test_parse_integer_literal_expression() {}
