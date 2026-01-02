mod inout;
mod ir;
mod lexer;
mod parser;
mod types;
mod vm;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::read_to_string;
use types::FlowListener;
fn main() {
    let vocabulary: inout::Vocabulary = inout::read_json("vocabulary.json".to_string());

    let filename:String  = std::env::args()
        .nth(1)
        .unwrap_or("".to_string());
    dbg!(&filename);
    if filename.trim().len() == 0 {
        println!("please provide a filename");
        return;
    }
    let mut tokens: Vec<types::Token> = lexer::tokenize_code(
        read_to_string(format!("code//{}.fq", filename)).unwrap_or("".to_string()),
        &vocabulary.keywords,
    );
    let tree: types::Statement =
        parser::parse_program(&mut tokens, &RefCell::new(FlowListener::Console));
    // inout::print_tree(tree.clone(), 0);
    let mut ir: Vec<ir::IR> = vec![];
    ir::ast_to_ir(&tree, &mut ir);
    // println!("IR: {:#?}", ir.iter().enumerate().collect::<Vec<(usize, &ir::IR)>>());
    let mut env: HashMap<usize,types::VarV> = HashMap::new();
    println!("output: ");
    vm::execute(ir.clone(), &mut env);
}
