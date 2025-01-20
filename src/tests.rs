use crate::environment::Environment;
use crate::inout::*;
use crate::lexer::tokenize_code;
use crate::old_runtime::run_statement;
use crate::old_runtime::VariableType;
use crate::parser::*;
use crate::types::ExpressionType;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
#[allow(dead_code)]
fn run_test_template_oneline(expr: String, expected_result: &mut Vec<VariableType>) {
    let tree: Rc<ExpressionType> = parse_program(&mut tokenize_code(expr));
    print_tree(tree.clone().into(), 0);
    for r in run_statement(
        tree.into(),
        Rc::new(RefCell::new(Environment::new(
            HashMap::new(),
            HashMap::new(),
        ))),
    ) {
        assert_eq!(r, expected_result.remove(0))
    }
}
#[allow(dead_code)]
fn run_test_template_file(name: &str, result: &mut Vec<VariableType>) {
    let code = read_file_contents(name).expect("cant read file while testing:");
    run_test_template_oneline(code, result);
}
#[test]
fn run_test_math() {
    run_test_template_oneline(
        String::from("(023 + 32) * 7 - 05 * (6 / 03)."),
        &mut vec![VariableType::Int(375)],
    );
}
#[test]
fn run_test_dynnamic_if() {
    run_test_template_oneline(
        String::from("{? 1000 >= 123 + 0234 234*5 + 2 !-? ==  354 + 234.}."),
        &mut vec![VariableType::Int(1172)],
    );
}
