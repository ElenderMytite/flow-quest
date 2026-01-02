use std::cell::RefCell;

use crate::vm::VarV;
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Block(Vec<Box<Statement>>),
    Set { name: usize, value: Box<Statement> },
    Nil,
    Name(usize),
    Bool(bool),
    Number(isize),
    Comparsion(ComparsionV, Box<Statement>, Box<Statement>),
    OperationBool(ActionV, Box<Statement>, Option<Box<Statement>>),
    OperationNumder(ActionV, Box<Statement>, Box<Statement>),
    If(Box<Statement>, Box<Statement>, Option<Box<Statement>>),
    Out { expr: Box<Statement>, to: RefCell<FlowListener> },
    In(RefCell<FlowStreamer>),
    Jump(bool),
}
#[derive(Debug, Clone, PartialEq)]
pub enum ActionV {
    Not,
    And,
    Or,
    Add,
    Sub,
    Div,
    Mul,
    Mod,
}
#[derive(Debug, Clone, PartialEq)]
pub enum ComparsionV {
    Equal,
    Less,
    Greater,
    NotEqual,
    LessOrEqual,
    GreaterOrEqual,
}
#[derive(PartialEq, Clone, Debug)]
pub enum FlowListener{
    Console,
}
impl FlowListener {
    pub fn get(&self,val: VarV) -> bool {
        match self {
            FlowListener::Console => {
                match val {
                    VarV::Num(val) => println!("{}",val),
                    VarV::Bool(val) => println!("{}",val),
                    VarV::Tuple(vec) => println!("{:?}",vec),   
                }
                true
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum FlowStreamer{
    Console,
    None
}
impl FlowStreamer {
    pub fn send(&self) -> VarV{
        match self {
            FlowStreamer::Console => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                VarV::Num(input.parse().unwrap())
            }
            FlowStreamer::None => VarV::Num(0),
        }
    }
}
