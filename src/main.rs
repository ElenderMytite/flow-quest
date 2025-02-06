mod flow;
mod inout;
mod intermediate_representation;
mod lexer;
mod parser;
mod tests;
mod types;
use inout::{ask_to_do_smth, get_way_to_run};
use parser::parse_program;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
fn main() {
    let mut tokens: Vec<types::TokenV> = lexer::tokenize_code(get_way_to_run());
    if ask_to_do_smth("print tokens") {
        println!("{:?}", tokens);
    }
    let tree: Rc<types::StatementV> = parse_program(&mut tokens);
    assert_eq!(tokens.len(), 0);
    if ask_to_do_smth("print tree") {
        inout::print_tree(tree.clone().into(), 0);
    }
    if ask_to_do_smth("convert to ir") {
        let mut ir = vec![];
        intermediate_representation::ast_to_ir(
            tree.clone().into(),
            &mut ir,
            &RefCell::new(flow::FlowListener::Console),
        );
        if ask_to_do_smth("print ir") {
            println!("{:#?}", &ir);
        }
        if ask_to_do_smth("run ir") {
            let mut env = HashMap::new();
            intermediate_representation::execute(ir, &mut env);
        }
    }
}
