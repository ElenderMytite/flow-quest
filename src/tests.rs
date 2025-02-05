use std::cell::RefCell;
use std::collections::HashMap;

use crate::{flow, inout::*};
use crate::intermediate_representation::{ast_to_ir, execute};
use crate::types::{TokenV, VarT};
use crate::lexer::tokenize_code;
use crate::parser::*;
use crate::types::Statement;
#[allow(dead_code)]
fn run_test_template_oneline(expr: String, expected_result: RefCell<Vec<VarT>>) {
    let mut tokens: Vec<TokenV> = tokenize_code(expr.clone());
    println!("{:?}", tokens);
    let tree: Statement = Statement{value: parse_program(&mut tokens)};
    print_tree(tree.clone().into(), 0);
    let mut ir = vec![];
    let listener = flow::FlowListener::Asserter(expected_result);
    ast_to_ir(tree, &mut ir,&RefCell::new(listener));
    let mut env = HashMap::new();
    execute(ir, &mut env);
}
#[allow(dead_code)]
fn run_test_template_file(name: &str, result: RefCell<Vec<VarT>>) {
    let code = read_file_contents(name).expect("cant read file while testing:");
    run_test_template_oneline(code, result);
}
#[test]
fn test_fib() {
    run_test_template_file(
        "fib",
        RefCell::new(vec![VarT::Num(0),VarT::Num(1),VarT::Num(1),VarT::Num(2),
                            VarT::Num(3),VarT::Num(5),VarT::Num(8),VarT::Num(13)]))
}
#[test]
fn test_math(){
    run_test_template_file("math", RefCell::new(vec![VarT::Num(128)]));
}
#[test]
fn test_if() {
    run_test_template_file("if", RefCell::new(vec![VarT::Bool(true)]));
}
