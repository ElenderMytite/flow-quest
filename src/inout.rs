use crate::types::{Statement, StatementV};
use serde::{Deserialize, Serialize};
use std::{
    borrow::Borrow,
    collections::HashMap,
    fs::File,
    io::{self, stdin, Read},
};
#[derive(Debug)]
pub struct Vocabulary {
    pub keywords: HashMap<String, u8>,
    #[allow(dead_code)]
    pub libwords: HashMap<String, ModulePiece>,
}
#[derive(Debug, Serialize, Deserialize)]
struct VocabularyBuilder {
    keywords: HashMap<String, u8>,
    libwords: HashMap<String, ModulePiece>,
    parent: Option<String>,
}
impl From<VocabularyBuilder> for Vocabulary {
    fn from(vb: VocabularyBuilder) -> Self {
        match vb.parent {
            None => Vocabulary {
                keywords: vb.keywords,
                libwords: vb.libwords,
            },
            Some(p) => {
                let parent = read_json(p + ".json");
                let mut keywords = parent.keywords;
                let mut libwords = parent.libwords;
                for (k, v) in vb.keywords {
                    keywords.insert(k, v);
                }
                for (k, v) in vb.libwords {
                    libwords.insert(k, v);
                }
                println!("{:#?}", keywords);
                Vocabulary { keywords, libwords }
            }
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ModulePiece {
    TransFormer,
    Streamer,
    Listener,
    Condition,
}
pub fn read_file_contents(filename: &str) -> io::Result<String> {
    let mut file = File::open(String::from("code/") + filename + ".nq")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
pub fn read_json(path: String) -> Vocabulary {
    let file = File::open(path).expect("cannot open file");
    let builder: VocabularyBuilder = serde_json::from_reader(file).expect("cannot read json");
    Vocabulary::from(builder)
}
pub fn get_way_to_run() -> String {
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
            let code = read_file_contents(path.as_str()).expect("cannot read file");
            code
        }
        _ => {
            println!("invalid option: {:?}; try again ", option);
            get_way_to_run()
        }
    }
}
pub fn ask_to_do_smth(text: &str) -> bool {
    let mut option: String = String::from("");
    println!("choose option: y: {text} n: do not {text} ");
    stdin().read_line(&mut option).expect("cannot readline");
    return option.as_str() == "y\r\n";
}

pub fn print_tree(node: Statement, depth: usize) {
    let indent = "\t".repeat(depth);
    match node.value.borrow() {
        StatementV::Number(val) => println!("{}Number: {}", indent, val),
        StatementV::OperationNumder(op, left, right) => {
            println!("{}Op int: {:?}", indent, op);
            print_tree(left.clone(), depth + 1);
            print_tree(right.clone(), depth + 1);
        }
        StatementV::OperationBool(op, left, right) => {
            println!("{}Op bool: {:?}", indent, op);
            print_tree(left.clone(), depth + 1);
            match right {
                Some(v) => print_tree(v.clone(), depth + 1),
                None => (),
            }
        }
        StatementV::Bool(val) => println!("{}Boolean: {}", indent, val),
        StatementV::Nil => println!("{}Nil", indent),

        StatementV::If(condition, if_, else_) => {
            println!("{}If: ", indent,);
            print_tree(condition.clone(), depth + 1);
            println!("{}Do:", indent);
            print_tree(if_.clone(), depth + 1);
            println!("{}else: ", indent);
            match else_ {
                Some(v) => print_tree(v.clone(), depth + 1),
                None => (),
            }
        }
        StatementV::Comparsion(comparsion_type, left, right) => {
            println!("{}Comparsion: {:?}", indent, comparsion_type);
            print_tree(left.clone(), depth + 1);
            print_tree(right.clone(), depth + 1);
        }
        StatementV::Block(vec, block_type) => {
            let capt: String = match block_type {
                crate::types::BlockV::Evaluate => String::from("Evaluate"),
                crate::types::BlockV::Draft => String::from("Draft"),
            };
            println!("{}{}:", indent, capt);
            for stmt in vec {
                print_tree(stmt.clone(), depth + 1);
            }
        }
        StatementV::OutExpr { expr, to } => {
            println!("{}Return:", indent);
            print_tree(expr.clone(), depth + 1);
            println!("{}To: {:?}", indent, *to);
        }
        StatementV::In(streamer) => {
            println!("{}Get from: {:?}", indent, *streamer);
        }
        StatementV::Name(name) => println!("{}Name: {:?}", indent, name),
        StatementV::Define { link, like } => {
            println!("{}Define:", indent);
            print_tree(link.clone(), depth + 1);
            println!("{}As: {}", indent, like);
        }
        StatementV::Assign(module_path, rc) => {
            println!("{}Assign:", indent);
            print_tree(rc.clone(), depth + 1);
            println!("{}To: {}", indent, module_path);
        }
        StatementV::Jump(up) => {
            let place = if *up {
                String::from("up")
            } else {
                String::from("down")
            };
            println!("{}Jump: {}", indent, place);
        }
        StatementV::Set { name, value } => {
            println!("{}Set to {}:", indent, name);
            print_tree(value.clone(), depth + 1);
        }
    }
}
