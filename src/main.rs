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
    let mut input: String = "".to_string() ;
    std::io::stdin().read_line(&mut input).expect("cannot read filename");
    let file = fs::read_to_string(format!("code/{}.fq", input.trim())).expect("cannot read file");
    let vocabulary: Vocabulary = read_json("vocabulary.json".to_string());
    let mut tokens: Vec<types::Token> = lexer::tokenize_code(file, &vocabulary.keywords);
    let tree: Rc<types::Statement> = Rc::new(parse_program(&mut tokens, &RefCell::new(FlowListener::Console)));
    let mut ir: Vec<ir::IR> = vec![];
    ir::ast_to_ir(&tree, &mut ir);
    let mut env: HashMap<usize, VarV> = HashMap::new();
    println!("output: ");
    vm::execute(ir.clone(), &mut env);
}
