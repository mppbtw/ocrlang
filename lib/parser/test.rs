use super::parse_from_string;
use crate::syntax::ExpressionType;
use crate::syntax::StatementType;

#[test]
fn test_parse_var_assign_statement() {
    let input = "a = 1
        global bb = 22
        ccc = 333";
    let prog = parse_from_string(input).unwrap();
    assert_eq!(prog.statements.len(), 3);

    assert_eq!(
        prog.statements[0].pretty_print(),
        "a = <PLACEHOLDER_EXPRESSION>"
    );

    assert_eq!(
        prog.statements[1].pretty_print(),
        "global bb = <PLACEHOLDER_EXPRESSION>"
    );

    assert_eq!(
        prog.statements[2].pretty_print(),
        "ccc = <PLACEHOLDER_EXPRESSION>"
    );
}

#[test]
fn test_parse_identifier_expression() {
    let input = "foo bar
        baz";
    let prog = parse_from_string(input).unwrap();
    assert_eq!(prog.statements.len(), 3);

    assert_eq!(prog.statements[0].pretty_print(), "foo");

    assert_eq!(prog.statements[1].pretty_print(), "bar");

    assert_eq!(prog.statements[2].pretty_print(), "baz");
}

#[test]
fn test_parse_return_statement() {
    let input = "return abc
        return";
    let prog = parse_from_string(input).unwrap();
    assert_eq!(prog.statements.len(), 2);

    assert_eq!(
        prog.statements[0].pretty_print(),
        "return <PLACEHOLDER_EXPRESSION>"
    );

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

#[test]
fn test_parse_bool_expression() {
    let input = "true true

            false";
    let prog = parse_from_string(input).unwrap();
    assert_eq!(prog.statements.len(), 3);
    assert!(matches!(
        prog.statements[0].get_type(),
        StatementType::Expression(_)
    ));
    if let StatementType::Expression(e) = prog.statements[0].get_type() {
        assert!(matches!(e.value.get_type(), ExpressionType::Boolean(_)))
    }
    assert_eq!(prog.statements[0].pretty_print(), "true");
    assert_eq!(prog.statements[1].pretty_print(), "true");
    assert_eq!(prog.statements[2].pretty_print(), "false");
}

#[test]
fn test_parse_prefix_expressions() {
    let input = "NOT true

        -5 +2";
    let prog = parse_from_string(input).unwrap();
    assert_eq!(prog.statements.len(), 3);
    assert_eq!(prog.statements[0].pretty_print(), "NOT true");
}

#[test]
fn test_parse_infix_expressions() {
    let input = "69 + 420
        69 - 420
        69 * 420
        69 / 420
        69 < 420
        69 <= 420
        69 > 420
        69 >= 420
        69 == 420
        69 MOD 420
        69 DIV 420
        69 != 420
        true OR false
        ";
    let prog = parse_from_string(input).unwrap();
    assert_eq!(prog.statements.len(), 13);

    assert_eq!(
        prog.statements
            .iter()
            .map(|stmt| stmt.pretty_print() + "\n")
            .collect::<Vec<String>>()
            .join("\n"),
        input
    );
}
