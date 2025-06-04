use core::ops::{Add, Div, Mul, Not, Sub, BitOr, BitAnd};
use std::cell::RefCell;
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::ops::Rem;
use std::rc::Rc;
use crate::ir::IR;
#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    pub value: Rc<StatementV>,
}
impl Statement {
    pub fn new(value: Rc<StatementV>) -> Self {
        Self { value }
    }
    pub fn get_ast(&self) -> StatementV {
        self.value.as_ref().clone()
    }
}
impl Into<Statement> for Rc<StatementV> {
    fn into(self) -> Statement {
        Statement::new(self)
    }
}
impl From<Statement> for Rc<StatementV> {
    fn from(statement: Statement) -> Rc<StatementV> {
        statement.value
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum StatementV {
    Block(Vec<Statement>, BlockV),
    Define { link: Statement, like: String },
    Call(Statement, Statement),
    Set { name: String, value: Statement },
    Nil,
    Name(String),
    Bool(bool),
    Number(isize),
    Comparsion(ComparsionV, Statement, Statement),
    OperationBool(ActionV, Statement, Option<Statement>),
    OperationNumder(ActionV, Statement, Statement),
    If(Statement, Statement, Option<Statement>),
    Out { expr: Statement, to: RefCell<FlowListener> },
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
#[derive(Debug, Clone, PartialEq)]
pub enum BlockV {
    Evaluate,
    Draft,
    Storage,
}
#[derive(Debug, Clone)]
pub enum VarV {
    Tuple(Vec<VarV>),
    Procedure(Vec<IR>),
    Num(isize),
    Bool(bool),
}
impl VarV {
    pub fn get_code(&self) -> Vec<IR> {
        match self {
            VarV::Procedure(code) => code.clone(),
            _ => panic!("Type mismatch: not a procedure"),
        }
    }
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
            VarV::Procedure(_) => todo!(),
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
#[derive(Debug, Clone, PartialEq)]
pub enum TokenV {
    Brackets { id: u8, is_opened: bool },
    Sign(u8),
    Bool(bool),
    Number(isize),
    Name(String),
    Mark(u8),
    Comparsion(u8),
    Dot(bool),
    EOF,
}
impl TokenV {
    pub fn get_int_value(&self) -> isize {
        match &self {
            TokenV::Number(v) => *v,
            TokenV::Name(_) => panic!("trying to get name"),
            _ => panic!("Unexpected token while trying to get int value"),
        }
    }
    pub fn get_string_from_name(&self) -> String {
        match &self {
            TokenV::Name(name) => name.clone(),
            _ => panic!("expected name"),
        }
    }
    pub fn is_operation(&self) -> bool {
        //print!("{:?}", self);
        match &self {
            TokenV::Mark(1 | 7 | 9) | TokenV::Comparsion(_) | TokenV::Sign(_) => true,
            _ => false,
        }
    }
    pub fn get_operation_priorety(&self) -> u8 {
        match &self {
            TokenV::Comparsion(_) => 4,
            TokenV::Sign(1..=2) => 5,
            TokenV::Sign(3..=5) => 6,
            TokenV::Mark(7) => 2,
            TokenV::Mark(9) => 1,
            TokenV::Mark(1) => 3,
            _ => 0,
        }
    }
    pub fn token_to_action_type(&self) -> ActionV {
        match &self {
            TokenV::Sign(1) => ActionV::Add,
            TokenV::Sign(2) => ActionV::Sub,
            TokenV::Sign(3) => ActionV::Mul,
            TokenV::Sign(4) => ActionV::Div,
            TokenV::Sign(5) => ActionV::Mod,
            TokenV::Mark(1) => ActionV::Not,
            TokenV::Mark(7) => ActionV::And,
            TokenV::Mark(9) => ActionV::Or,
            _ => panic!("invalid action type"),
        }
    }
    pub fn token_to_comparsion_type(&self) -> ComparsionV {
        match &self {
            TokenV::Comparsion(id) => match id {
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
    Asserter(RefCell<Vec<VarV>>),
    None
}
impl FlowListener {
    pub fn get(&self,val: VarV) -> bool {
        match self {
            FlowListener::Console => {
                match val {
                    VarV::Num(val) => println!("{}",val),
                    VarV::Bool(val) => println!("{}",val),
                    VarV::Tuple(vec) => println!("{:?}",vec),
                    VarV::Procedure(_) => println!("Procedure"),                 
                }
                true
            }
            FlowListener::Asserter(expected_values) => {
                expected_values.borrow_mut().remove(0) == val 
            }
            FlowListener::None => true,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
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
