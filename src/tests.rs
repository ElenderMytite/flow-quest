use std::cell::RefCell;
use std::collections::HashMap;

use crate::{flow, inout::*};
use crate::intermediate_representation::{ast_to_ir, execute};
use crate::types::StackVarType;
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
fn test_fib() {
    run_test_template_file(
        "fib_fourth",
        RefCell::new(vec![StackVarType::Num(0),StackVarType::Num(1),StackVarType::Num(1),StackVarType::Num(2),
                            StackVarType::Num(3),StackVarType::Num(5),StackVarType::Num(8),StackVarType::Num(13)]))
}
#[test]
fn test_math(){
    run_test_template_file("math", RefCell::new(vec![StackVarType::Num(128)]));
}
#[test]
fn test_if() {
    run_test_template_file("if", RefCell::new(vec![StackVarType::Bool(true)]));
}
