use std::cell::RefCell;
use std::collections::HashMap;

use crate::ir::{ast_to_ir, execute};
use crate::lexer::tokenize_code;
use crate::parser::*;
use crate::types::{FlowListener, Statement};
use crate::types::{TokenV, VarV};
use crate::inout::*;

#[allow(dead_code)]
fn run_test_template_oneline(expr: String, expected_result: RefCell<Vec<VarV>>) {
    let vocab = read_json("vocabulary.json".to_string());
    let mut tokens: Vec<TokenV> = tokenize_code(expr.clone(), &vocab.keywords);
    println!("{:?}", tokens);
    let tree: Statement = Statement {
        value: parse_program(&mut tokens, &RefCell::new(FlowListener::Asserter(expected_result))),
    };
    print_tree(tree.clone().into(), 0);
    let mut ir = vec![];
    ast_to_ir(tree, &mut ir);
    println!("{:?}", ir.iter().enumerate().collect::<Vec<_>>());
    let mut env = HashMap::new();
    execute(ir, &mut env);
}
#[allow(dead_code)]
fn run_test_template_file(name: &str, result: RefCell<Vec<VarV>>) {
    let code = read_file_contents(name).expect("cant read file while testing:");
    run_test_template_oneline(code, result);
}
#[test]
fn test_fib() {
    run_test_template_file(
        "fib",
        RefCell::new(vec![
            VarV::Num(1),
            VarV::Num(1),
            VarV::Num(2),
            VarV::Num(3),
            VarV::Num(5),
            VarV::Num(8),
            VarV::Num(13),
            VarV::Num(21),
        ]),
    )
}
#[test]
fn test_math() {
    run_test_template_file("math", 
    RefCell::new(vec![VarV::Num(128)])
    );
}
#[test]
fn test_if() {
    run_test_template_file("if", 
    RefCell::new(vec![VarV::Bool(true)])
    );
}

#[test]
fn test_operation_order() {
    run_test_template_file("order", 
    RefCell::new(vec![VarV::Num(345)])
    );
}

