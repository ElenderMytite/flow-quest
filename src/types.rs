use core::ops::{Add, Div, Mul, Not, Sub, BitOr, BitAnd};
use std::cell::RefCell;
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::ops::Rem;
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
#[derive(Debug, Clone)]
pub enum VarV {
    Tuple(Vec<VarV>),
    Num(isize),
    Bool(bool),
}
impl VarV {
}
impl Eq for VarV {}
impl PartialEq for VarV {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (VarV::Num(a), VarV::Num(b)) => a == b,
            (VarV::Bool(a), VarV::Bool(b)) => a == b,
            (VarV::Tuple(t1), VarV::Tuple(t2)) => t1 == t2,
            _ => false,
        }
    }
}
impl Ord for VarV {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (VarV::Num(a), VarV::Num(b)) => a.cmp(b),
            _ => panic!("Type mismatch"),
        }
    }
}
impl PartialOrd for VarV {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (VarV::Num(a), VarV::Num(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}
impl Not for VarV {
    type Output = Self;
    fn not(self) -> Self {
        match self {
            VarV::Bool(b) => VarV::Bool(!b),
            VarV::Num(v) => VarV::Num(-v),
            VarV::Tuple(_) => todo!(),
        }
    }
}
impl Add for VarV {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (VarV::Num(a), VarV::Num(b)) => VarV::Num(a + b),
            (VarV::Bool(a), VarV::Bool(b)) => VarV::Bool(a || b),
            _ => panic!("Type mismatch"),
        }
    }
}
impl Sub for VarV {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (VarV::Num(a), VarV::Num(b)) => VarV::Num(a - b),
            _ => panic!("Type mismatch"),
        }
    }
}
impl Mul for VarV {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (VarV::Num(a), VarV::Num(b)) => VarV::Num(a * b),
            (VarV::Bool(a), VarV::Bool(b)) => VarV::Bool(a && b),
            _ => panic!("Type mismatch"),
        }
    }
}
impl Div for VarV {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        match (self, other) {
            (VarV::Num(a), VarV::Num(b)) => VarV::Num(a / b),
            _ => panic!("Type mismatch"),
        }
    }
}
impl Rem for VarV {
    type Output = Self;
    fn rem(self, other: Self) -> Self {
        match (self, other) {
            (VarV::Num(a), VarV::Num(b)) => VarV::Num(a % b),
            _ => panic!("Type mismatch"),
        }
    }
    
}
impl BitOr for VarV {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        match (self, other) {
            (VarV::Bool(a), VarV::Bool(b)) => VarV::Bool(a || b),
            _ => panic!("Type mismatch"),
        }
    }
}
impl BitAnd for VarV {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        match (self, other) {
            (VarV::Bool(a), VarV::Bool(b)) => VarV::Bool(a && b),
            _ => panic!("Type mismatch"),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Brackets { id: u8, is_opened: bool },
    Sign(u8),
    Bool(bool),
    Number(isize),
    Name(usize),
    Mark(u8),
    Comparsion(u8),
    Dot(bool),
    EOF,
}
impl Token {
    pub fn is_operation(&self) -> bool {
        //print!("{:?}", self);
        match &self {
            Token::Mark(1 | 7 | 9) | Token::Comparsion(_) | Token::Sign(_) => true,
            _ => false,
        }
    }
    pub fn name_id(&self) -> usize {
        match &self {
            Token::Name(id) => *id,
            _ => panic!("expected name token"),
        }
    }
    pub fn get_operation_priorety(&self) -> u8 {
        match &self {
            Token::Comparsion(_) => 4,
            Token::Sign(1..=2) => 5,
            Token::Sign(3..=5) => 6,
            Token::Mark(7) => 2,
            Token::Mark(9) => 1,
            Token::Mark(1) => 3,
            _ => 0,
        }
    }
    pub fn token_to_action_type(&self) -> ActionV {
        match &self {
            Token::Sign(1) => ActionV::Add,
            Token::Sign(2) => ActionV::Sub,
            Token::Sign(3) => ActionV::Mul,
            Token::Sign(4) => ActionV::Div,
            Token::Sign(5) => ActionV::Mod,
            Token::Mark(1) => ActionV::Not,
            Token::Mark(7) => ActionV::And,
            Token::Mark(9) => ActionV::Or,
            _ => panic!("invalid action type"),
        }
    }
    pub fn token_to_comparsion_type(&self) -> ComparsionV {
        match &self {
            Token::Comparsion(id) => match id {
                1 => ComparsionV::Equal,
                2 => ComparsionV::Greater,
                3 => ComparsionV::Less,
                4 => ComparsionV::NotEqual,
                5 => ComparsionV::GreaterOrEqual,
                6 => ComparsionV::LessOrEqual,
                _ => panic!("invalid comparsion type id"),
            },
            _ => panic!("expected comparsion"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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
