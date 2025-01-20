use std::cell::RefCell;
use std::collections::HashMap;

use crate::{flow, inout::*};
use crate::intermediate_representation::{ast_to_ir, execute, StackVarType};
use crate::lexer::tokenize_code;
use crate::parser::*;
use crate::types::Statement;
#[allow(dead_code)]
fn run_test_template_oneline(expr: String, expected_result: RefCell<Vec<StackVarType>>) {
    let tree: Statement = Statement{value: parse_program(&mut tokenize_code(expr))};
    print_tree(tree.clone().into(), 0);
    let mut ir = vec![];
    let listener = flow::FlowListener::Asserter(expected_result);
    ast_to_ir(tree, &mut ir,&RefCell::new(listener));
    let mut env = HashMap::new();
    execute(ir, &mut env);
}
#[allow(dead_code)]
fn run_test_template_file(name: &str, result: RefCell<Vec<StackVarType>>) {
    let code = read_file_contents(name).expect("cant read file while testing:");
    run_test_template_oneline(code, result);
}
#[test]
fn run_test_math() {
    run_test_template_oneline(
        String::from("(023 + 32) * 7 - 05 * (6 / 03)."),
        RefCell::new(vec![StackVarType::Num(375)]),
    );
}
#[test]
fn run_test_dynnamic_if() {
    run_test_template_oneline(
        String::from("{? 1000 >= 123 + 0234 234*5 + 2 !-? ==  354 + 234.}."),
        RefCell::new(vec![StackVarType::Num(1172)]),
    );
}
