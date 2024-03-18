use super::parse_from_string;
use crate::syntax::PrettyPrint;
use crate::syntax::StatementType;

#[test]
fn test_parse_var_assign_statement() {
    let input = "a = 1
        global bb = 22
        ccc = 333";
    let prog = parse_from_string(input).unwrap();
    assert_eq!(prog.statements.len(), 3);

    assert_eq!(prog.statements[0].pretty_print(), "a = <PLACEHOLDER_EXPRESSION>");

    assert_eq!(prog.statements[1].pretty_print(), "global bb = <PLACEHOLDER_EXPRESSION>");

    assert_eq!(prog.statements[2].pretty_print(), "ccc = <PLACEHOLDER_EXPRESSION>");
}

#[test]
fn test_parse_return_statement() {
    let input = "return abc
        return";
    let prog = parse_from_string(input).unwrap();
    assert_eq!(prog.statements.len(), 2);

    assert_eq!(prog.statements[0].pretty_print(), "return <PLACEHOLDER_EXPRESSION>");

    assert_eq!(prog.statements[1].pretty_print(), "return");
}

#[test]
fn test_parse_integer_literal_expression() {
    let input = "123 456";
    let prog = parse_from_string(input).unwrap();
    assert_eq!(prog.statements.len(), 2);
    assert_eq!(prog.statements[0].pretty_print(), "123");
    assert_eq!(prog.statements[1].pretty_print(), "456");
}
