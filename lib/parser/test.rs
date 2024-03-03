use super::Parser;
use crate::lexer::Lexer;
use crate::parser::ast::Statement;

#[test]
fn test_parse_var_assign_statement() {
    let input = "a = 1
        global bb = 22
        ccc = 333";
    let mut parser = Parser::new(Lexer::new(input)).unwrap();
    let prog = parser.parse().unwrap();
    assert_eq!(prog.statements.len(), 3);

    let a = &prog.statements[0];
    assert!(matches!(a, Statement::Assign { .. }));
    if let Statement::Assign { ident, global, .. } = a {
        assert_eq!(ident.get_ident(), "a");
        assert!(!global);
    }

    let b = &prog.statements[1];
    assert!(matches!(b, Statement::Assign { .. }));
    if let Statement::Assign { ident, global, .. } = b {
        assert_eq!(ident.get_ident(), "bb");
        assert!(global);
    }

    let c = &prog.statements[2];
    assert!(matches!(c, Statement::Assign { .. }));
    if let Statement::Assign { ident, global, .. } = c {
        assert_eq!(ident.get_ident(), "ccc");
        assert!(!global);
    }
}

#[test]
fn test_parse_return_statement() {
    let input = "return a+b
        return c+d";
    let mut parser = Parser::new(Lexer::new(input)).unwrap();
    let prog = parser.parse().unwrap();
    assert_eq!(prog.statements.len(), 2);

    for s in prog.statements {
        assert!(matches!(s, Statement::Return { .. }));
    }
}
