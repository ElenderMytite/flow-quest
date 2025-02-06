use core::ops::{Add, Div, Mul, Not, Sub};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::rc::Rc;

use crate::intermediate_representation::IR;
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
pub type Path = Option<String>;
#[derive(Debug, Clone, PartialEq)]
pub enum StatementV {
    Block(Vec<Statement>, BlockV),
    Define { link: Statement, like: String },
    Assign(String, Statement),
    Set { name: String, value: Statement },
    Nil,
    Name(String),
    Bool(bool),
    Number(isize),
    Comparsion(ComparsionV, Statement, Statement),
    OperationBool(ActionV, Statement, Option<Statement>),
    OperationNumder(ActionV, Statement, Statement),
    If(Statement, Statement, Option<Statement>),
    OutExpr { expr: Statement, like: Path },
    In(String),
    Jump(bool),
}
#[derive(Debug, Clone, PartialEq)]
pub enum ActionV {
    Not,
    And,
    Or,
    Plus,
    Minus,
    Divide,
    Multiply,
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
}
#[derive(Debug, Clone)]
pub enum VarT {
    Tuple(Vec<VarT>),
    Procedure(Vec<IR>),
    Num(isize),
    Bool(bool),
}
impl VarT {
    pub fn get_code(&self) -> Vec<IR> {
        match self {
            VarT::Procedure(code) => code.clone(),
            _ => panic!("Type mismatch: not a procedure"),
        }
    }
}
impl Eq for VarT {}
impl PartialEq for VarT {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (VarT::Num(a), VarT::Num(b)) => a == b,
            (VarT::Bool(a), VarT::Bool(b)) => a == b,
            (VarT::Tuple(t1), VarT::Tuple(t2)) => t1 == t2,
            _ => false,
        }
    }
}
impl Ord for VarT {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (VarT::Num(a), VarT::Num(b)) => a.cmp(b),
            _ => panic!("Type mismatch"),
        }
    }
}
impl PartialOrd for VarT {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (VarT::Num(a), VarT::Num(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}
impl Not for VarT {
    type Output = Self;
    fn not(self) -> Self {
        match self {
            VarT::Bool(b) => VarT::Bool(!b),
            VarT::Num(v) => VarT::Num(-v),
            VarT::Tuple(_) => todo!(),
            VarT::Procedure(_) => todo!(),
        }
    }
}
impl Add for VarT {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (VarT::Num(a), VarT::Num(b)) => VarT::Num(a + b),
            (VarT::Bool(a), VarT::Bool(b)) => VarT::Bool(a || b),
            _ => panic!("Type mismatch"),
        }
    }
}
impl Sub for VarT {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (VarT::Num(a), VarT::Num(b)) => VarT::Num(a - b),
            _ => panic!("Type mismatch"),
        }
    }
}
impl Mul for VarT {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (VarT::Num(a), VarT::Num(b)) => VarT::Num(a * b),
            (VarT::Bool(a), VarT::Bool(b)) => VarT::Bool(a && b),
            _ => panic!("Type mismatch"),
        }
    }
}
impl Div for VarT {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        match (self, other) {
            (VarT::Num(a), VarT::Num(b)) => VarT::Num(a / b),
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
    Keyword(WordT),
    Comparsion(u8),
    Dot(bool),
    EOF,
}
impl TokenV {
    pub fn get_int_value(&self) -> isize {
        match &self {
            TokenV::Number(v) => *v,
            TokenV::Name(_) => panic!("trying to get name"),
            TokenV::Keyword(_) => panic!("trying to get keyword"),
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
            TokenV::Comparsion(_) => 1,
            TokenV::Sign(1..=2) => 5,
            TokenV::Sign(3..=4) => 6,
            TokenV::Mark(7) => 2,
            TokenV::Mark(9) => 3,
            TokenV::Mark(1) => 4,
            _ => 0,
        }
    }
    pub fn token_to_action_type(&self) -> ActionV {
        match &self {
            TokenV::Sign(1) => ActionV::Plus,
            TokenV::Sign(2) => ActionV::Minus,
            TokenV::Sign(3) => ActionV::Multiply,
            TokenV::Sign(4) => ActionV::Divide,
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
pub enum WordT {
    In,
    Out,
    Go,
    Stop,
    Again,
}
