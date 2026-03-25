use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
};

use crate::types::Statement;
#[derive(Debug)]
pub struct Vocabulary {
    pub keywords: HashMap<String, u8>,
}
#[derive(Debug, Serialize, Deserialize)]
struct VocabularyBuilder {
    keywords: HashMap<String, u8>,
    parent: Option<String>,
}
impl From<VocabularyBuilder> for Vocabulary {
    fn from(vb: VocabularyBuilder) -> Self {
        match vb.parent {
            None => Vocabulary {
                keywords: vb.keywords,
            },
            Some(p) => {
                let parent = read_json(p + ".json");
                let mut keywords = parent.keywords;
                for (k, v) in vb.keywords {
                    keywords.insert(k, v);
                }
                Vocabulary {keywords}
            }
        }
    }
}
pub fn read_json(path: String) -> Vocabulary {
    let file = File::open(path).expect("cannot open file");
    let builder: VocabularyBuilder = serde_json::from_reader(file).expect("cannot read json");
    Vocabulary::from(builder)
}
#[allow(dead_code)]
pub fn print_tree(node: Statement, depth: usize) {
    let indent = "\t".repeat(depth);
    match node {
        Statement::Number(val) => println!("{}Number: {}", indent, val),
        Statement::OperationNumder(op, left, right) => {
            println!("{}{:?}", indent, op);
            print_tree(*left, depth + 1);
            print_tree(*right, depth + 1);
        }
        Statement::OperationBool(op, left, right) => {
            println!("{}{:?}", indent, op);
            print_tree(*left, depth + 1);
            match right {
                Some(v) => print_tree(*v, depth + 1),
                None => (),
            }
        }
        Statement::Bool(val) => println!("{}Boolean: {}", indent, val),
        Statement::Nil => (),

        Statement::If(condition, if_, else_) => {
            println!("{}If: ", indent,);
            print_tree(*condition, depth + 1);
            println!("{}Do:", indent);
            print_tree(*if_, depth + 1);
            println!("{}Else do: ", indent);
            match else_ {
                Some(v) => print_tree(*v, depth + 1),
                None => (),
            }
        }
        Statement::Comparsion(comparsion_type, left, right) => {
            println!("{}{:?}", indent, comparsion_type);
            print_tree(*left, depth + 1);
            print_tree(*right, depth + 1);
        }
        Statement::Block(vec) => {
            for stmt in vec {
                print_tree(*stmt, depth);
            }
        }
        Statement::Out { expr, to } => {
            println!("{}Return:", indent);
            print_tree(*expr, depth + 1);
            println!("{}To: {:?}", indent, to);
        }
        Statement::In(streamer) => {
            println!("{}Get from: {:?}", indent, streamer);
        }
        Statement::Name(name) => println!("{}Name: {:?}", indent, name),
        Statement::Jump(up) => {
            let place = if up {
                String::from("up")
            } else {
                String::from("down")
            };
            println!("{}Jump: {}", indent, place);
        }
        Statement::Set { name, value } => {
            println!("{}Set to {}:", indent, name);
            print_tree(*value, depth + 1);
        }
    }
}
