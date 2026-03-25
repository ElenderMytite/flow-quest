mod inout;
mod ir;
mod lexer;
mod parser;
mod vm;
mod types;
use crate::types::VarV;
use inout::{read_json, Vocabulary};
use parser::parse_program;
use types::FlowListener;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::rc::Rc;
fn main() {
    let file = fs::read_to_string("test.code").expect("cannot read file");
    let vocabulary: Vocabulary = read_json("vocabulary.json".to_string());
    let mut tokens: Vec<types::Token> = lexer::tokenize_code(file, &vocabulary.keywords);
    let tree: Rc<types::Statement> = Rc::new(parse_program(&mut tokens, &RefCell::new(FlowListener::Console)));
    assert_eq!(tokens.len(), 0);
    let mut ir: Vec<ir::IR> = vec![];
    ir::ast_to_ir(&tree, &mut ir);
    let mut env: HashMap<usize, VarV> = HashMap::new();
    println!("output: ");
    vm::execute(ir.clone(), &mut env);
}
