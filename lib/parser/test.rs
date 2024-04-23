use super::parse_from_string;
use crate::syntax::StatementType;

#[test]
fn test_parse_var_assign_statement() {
    let input = "a = 1
global bb = 22
ccc = 333";
    let prog = parse_from_string(input).unwrap();
    assert_eq!(prog.statements.len(), input.lines().count());
    assert_eq!(
        prog.statements
            .iter()
            .map(|stmt| stmt.pretty_print())
            .collect::<Vec<String>>()
            .join("\n"),
        input
    );
}

#[test]
fn test_parse_identifier_expr() {
    let input = "foo
bar
baz";
    let prog = parse_from_string(input).unwrap();
    assert_eq!(prog.statements.len(), input.lines().count());
    assert_eq!(
        prog.statements
            .iter()
            .map(|stmt| stmt.pretty_print())
            .collect::<Vec<String>>()
            .join("\n"),
        input
    );
}

#[test]
fn test_parse_return_statement() {
    let input = "return abc
return";
    let prog = parse_from_string(input).unwrap();
    assert_eq!(prog.statements.len(), input.lines().count());
    assert_eq!(
        prog.statements
            .iter()
            .map(|stmt| stmt.pretty_print())
            .collect::<Vec<String>>()
            .join("\n"),
        input
    );
}

#[test]
fn test_parse_number_literal_expr() {
    let input = "123
456";
    let prog = parse_from_string(input).unwrap();
    assert_eq!(prog.statements.len(), input.lines().count());
    assert_eq!(
        prog.statements
            .iter()
            .map(|stmt| stmt.pretty_print())
            .collect::<Vec<String>>()
            .join("\n"),
        input
    );
}

#[test]
fn test_parse_bool_expr() {
    let input = "true
true
false";
    let prog = parse_from_string(input).unwrap();
    assert_eq!(prog.statements.len(), input.lines().count());
    assert_eq!(
        prog.statements
            .iter()
            .map(|stmt| stmt.pretty_print())
            .collect::<Vec<String>>()
            .join("\n"),
        input
    );
}

#[test]
fn test_parse_prefix_expressions() {
    let input = "NOT true
-5
+2";
    let prog = parse_from_string(input).unwrap();
    assert_eq!(prog.statements.len(), input.lines().count());
    assert_eq!(
        prog.statements
            .iter()
            .map(|stmt| stmt.pretty_print())
            .collect::<Vec<String>>()
            .join("\n"),
        input
    );
}

#[test]
fn test_parse_infix_expressions() {
    let input = "69+420
69-420
69*420
69/420
69<420
69<=420
69>420
69>=420
69==420
69 MOD 420
69 DIV 420
69!=420
true OR false";
    let prog = parse_from_string(input).unwrap();
    assert_eq!(prog.statements.len(), input.lines().count());

    assert_eq!(
        prog.statements
            .iter()
            .map(|stmt| stmt.pretty_print())
            .collect::<Vec<String>>()
            .join("\n"),
        input
    );
}

#[test]
fn test_infix_expression_precedence() {
    let input = [
        ["5+5", "(5+5)"],
        ["-a+b", "((-a)+b)"],
        ["-a+b * NOT c", "((-a)+(b*(NOT c)))"],
        ["5*5+5", "((5*5)+5)"],
        ["5+5*5", "(5+(5*5))"],
        ["5-5/5", "(5-(5/5))"],
        ["5-5/5+5", "((5-(5/5))+5)"],
        ["5 MOD 5+5", "((5 MOD 5)+5)"],
        ["5 MOD 5*5", "((5 MOD 5)*5)"],
        ["5 MOD 5*5", "((5 MOD 5)*5)"],
    ];
    let input_lines = input.map(|l| l[0]).join("\n");
    let prog = parse_from_string(&input_lines).unwrap();
    assert_eq!(prog.statements.len(), input.len());

    for (i, line) in input.iter().enumerate() {
        assert!(matches!(
            prog.statements[i].get_type(),
            StatementType::Expression(_)
        ));
        if let StatementType::Expression(x) = prog.statements[i].get_type() {
            assert_eq!(x.value.pretty_print_with_brackets(), line[1]);
        }
    }
}

#[test]
fn test_parse_expr_with_paren() {
    let input = [
        ["5 * (5 + 5)", "(5*(5+5))"],
        ["5 + 5 * 5", "(5+(5*5))"],
        ["5 + (5 * 5)", "(5+(5*5))"],
        ["NOT (true == false)", "(NOT (true==false))"],
    ];
    let input_lines = input.map(|l| l[0]).join("\n");
    let prog = parse_from_string(&input_lines).unwrap();
    assert_eq!(prog.statements.len(), input.len());

    for (i, line) in input.iter().enumerate() {
        assert!(matches!(
            prog.statements[i].get_type(),
            StatementType::Expression(_)
        ));
        if let StatementType::Expression(x) = prog.statements[i].get_type() {
            assert_eq!(x.value.pretty_print_with_brackets(), line[1]);
        }
    }
}
