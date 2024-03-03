use super::Parser;
use crate::lexer::Lexer;
use crate::parser::ast::Statement;

#[test]
fn test_parse_var_assign_statement() {
    let input = "a = 1
        bb = 22
        ccc = 333";
    let mut parser = Parser::new(Lexer::new(input)).unwrap();
    let prog = parser.parse().unwrap();
    assert_eq!(prog.statements.len(), 3);

    let a = &prog.statements[0];
    assert!(matches!(a, Statement::Assign { .. }));
    if let Statement::Assign { ident, .. } = a {
        assert_eq!(ident.get_ident(), "a");
    }

    let b = &prog.statements[1];
    assert!(matches!(b, Statement::Assign { .. }));
    if let Statement::Assign { ident, .. } = b {
        assert_eq!(ident.get_ident(), "bb");
    }

    let c = &prog.statements[2];
    assert!(matches!(c, Statement::Assign { .. }));
    if let Statement::Assign { ident, .. } = c {
        assert_eq!(ident.get_ident(), "ccc");
    }
}
