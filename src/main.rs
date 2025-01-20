mod parser;
mod inout;
mod lexer;
mod types;
mod flow;
mod intermediate_representation;
mod tests;
use parser::parse_program;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::stdin;
use std::rc::Rc;

fn get_way_to_run() -> String {
    let mut option: String = String::from("");
    let mut code: String = String::from("");
    let mut path: String = String::from("");
    println!("choose option: 1. code in terminal 2.read code from file ");
    stdin().read_line(&mut option).expect("cannot readline");
    match option.as_str() {
        "1\r\n" => {
            println!("please input code: ");
            stdin().read_line(&mut code).expect("cannot codeline");
            code
        }
        "2\r\n" => {
            println!("please input file name (without format): ");
            stdin().read_line(&mut path).expect("cannot readpath");
            path.truncate(path.len() - 2);
            let code = inout::read_file_contents(path.as_str()).expect("cannot read file");
            code 
        }
        _ => {
            println!("invalid option: {:?}; try again ", option);
            get_way_to_run()
        }
    }
}

fn ask_to_do_smth(text: &str, ) -> bool {
    let mut option: String = String::from("");
    println!("choose option: y: {text} n: do not {text} ");
    stdin().read_line(&mut option).expect("cannot readline");
    return option.as_str() == "y\r\n";
}
fn main() {
    let mut tokens: Vec<lexer::Token> = lexer::tokenize_code(get_way_to_run());
    if ask_to_do_smth("print tokens")
    {println!("{:?}", tokens);}
    let tree: Rc<types::ExpressionType> = parse_program(&mut tokens);
    assert_eq!(tokens.len(), 0);
    if ask_to_do_smth("print tree")
    {inout::print_tree(tree.clone().into(), 0);}
    if ask_to_do_smth("convert to ir")
    {
        let mut ir = vec![]; 
        intermediate_representation::ast_to_ir(tree.clone().into(),&mut ir, &RefCell::new(flow::FlowListener::Console));
        if ask_to_do_smth("print ir"){println!("{:#?}",&ir);}
        if ask_to_do_smth("run ir")
        {
            let mut env = HashMap::new();
            intermediate_representation::execute(ir, &mut env);
        }
    }
}
