use super::parse_from_string;
use crate::syntax::PrettyPrint;
use crate::syntax::StatementType;

#[test]
fn test_parse_var_assign_statement() {
    let input = "a=1
global bb=22
ccc=333";
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

#[test]
fn test_parse_if_statement() {
    let input = [
        "if true != false then
            x = 5
            y = x - 5
        else 
            y = 5
            x = y - 5
        endif",
        "if x + y > 5 then
            x = y + 5
        endif",
    ];
    let input_lines = input.join("\n");
    let prog = parse_from_string(&input_lines).unwrap();
    assert_eq!(prog.statements.len(), input.len());

    assert!(matches!(
        prog.statements[0].get_type(),
        StatementType::If(_)
    ));
    if let StatementType::If(i) = prog.statements[0].get_type() {
        assert_eq!(i.condition.pretty_print(), "true!=false");
        assert_eq!(i.consequence.pretty_print(), "x=5\ny=x-5");
        assert!(i.alternative.is_some());
        assert_eq!(i.alternative.as_ref().unwrap().pretty_print(), "y=5\nx=y-5");
    }

    assert!(matches!(
        prog.statements[1].get_type(),
        StatementType::If(_)
    ));
    if let StatementType::If(i) = prog.statements[1].get_type() {
        assert_eq!(i.condition.pretty_print(), "x+y>5");
        assert_eq!(i.consequence.pretty_print(), "x=y+5");
    }
}

#[test]
fn test_parse_function() {
    let input = [
        "function my_func(arg1, arg2)
            x = 1
            y = 2
            x = x + 1
            y = y + 1
        endfunction",
        "procedure my_proc()
            y = 1 + 2 + 3 + 4 + 5
            y = -1/12
        endprocedure",
    ];
    let input_lines = input.join("\n");
    let prog = parse_from_string(&input_lines).unwrap();
    assert_eq!(prog.statements.len(), input.len());

    assert!(matches!(
        prog.statements[0].get_type(),
        StatementType::Function(_)
    ));
    if let StatementType::Function(i) = prog.statements[0].get_type() {
        assert_eq!(
            i.pretty_print(),
            "function my_func(arg1, arg2)
x=1
y=2
x=x+1
y=y+1
endfunction"
        )
    }

    assert!(matches!(
        prog.statements[1].get_type(),
        StatementType::Function(_)
    ));
    if let StatementType::Function(i) = prog.statements[1].get_type() {
        assert_eq!(
            i.pretty_print(),
            "procedure my_proc()
y=1+2+3+4+5
y=-1/12
endprocedure"
        )
    }
}

#[test]
fn test_parse_function_call() {
    let input = "x=my_function()
y=my_function(1, x+3, y)";
    let prog = parse_from_string(input).unwrap();

    assert_eq!(prog.statements[0].pretty_print(), "x=my_function()");
    assert_eq!(prog.statements[1].pretty_print(), "y=my_function(1, x+3, y)");

}
