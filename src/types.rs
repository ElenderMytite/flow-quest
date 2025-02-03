use std::rc::Rc;
use core::ops::{Add, Div, Mul, Not, Sub};
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

use crate::intermediate_representation::IR;
#[derive(Debug,Clone,PartialEq)]
pub struct Statement
{
    pub value: Rc<ExpressionType>,
}
impl Statement {
    pub fn new(value: Rc<ExpressionType>, ) -> Self {
        Self { value}
    }
    pub fn get_ast(&self) -> ExpressionType {
        self.value.as_ref().clone()
    }
}
impl Into<Statement> for Rc<ExpressionType> {
    fn into(self) -> Statement {
        Statement::new(self)
    }
}
impl From<Statement> for Rc<ExpressionType> {
    fn from(statement: Statement) -> Rc<ExpressionType> {
        statement.value
    }
    
}
pub type Path = Option<String>;
#[derive(Debug,Clone,PartialEq)]
pub enum ExpressionType {
    Block(Vec<Statement>,BlockType),
    Define{link: Statement,like: String},
    Assign(String, Statement),
    Set{name: String,value: Statement},
    Nil,
    Name(String),
    Bool(bool),
    Number(isize),
    Comparsion(ComparsionType, Statement, Statement),
    OperationBool(ActionType,Statement,Option<Statement>),
    OperationNumder(ActionType, Statement, Statement),
    If(Statement, Statement, Option<Statement>),
    OutExpr { expr: Statement, like: Path },
    In(String),
    Jump(bool),
}
#[derive(Debug,Clone,PartialEq)]
pub enum ActionType {
    Not,
    And,
    Or,
    Plus,
    Minus,
    Divide,
    Multiply,
}
#[derive(Debug,Clone,PartialEq)]
pub enum ComparsionType {
    Equal,
    Less,
    Greater,
    NotEqual,
    LessOrEqual,
    GreaterOrEqual,
}
#[derive(Debug,Clone,PartialEq)]
pub enum BlockType {
    Evaluate,
    Draft,
}
#[derive(Debug,Clone)]
pub enum StackVarType {
    Tuple(Vec<StackVarType>),
    Procedure(Vec<IR>),
    Num(isize),
    Bool(bool),
}
impl StackVarType {
    pub fn get_code(&self) -> Vec<IR>{
        match self {
            StackVarType::Procedure(code) => code.clone(),
            _ => panic!("Type mismatch: not a procedure"),
        }
    }
    
}
impl Eq for StackVarType {}
impl PartialEq for StackVarType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (StackVarType::Num(a), StackVarType::Num(b)) => a == b,
            (StackVarType::Bool(a), StackVarType::Bool(b)) => a == b,
            (StackVarType::Tuple(t1), StackVarType::Tuple(t2)) => t1 == t2,
            _ => false,
        }
    }
}
impl Ord for StackVarType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (StackVarType::Num(a), StackVarType::Num(b)) => a.cmp(b),
            _ => panic!("Type mismatch"),
        }
    }
}
impl PartialOrd for StackVarType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (StackVarType::Num(a), StackVarType::Num(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}
impl Not for StackVarType {
    type Output = Self;
    fn not(self) -> Self {
        match self {
            StackVarType::Bool(b) => StackVarType::Bool(!b),
            StackVarType::Num(v) => StackVarType::Num(-v),
            StackVarType::Tuple(_) => todo!(),
            StackVarType::Procedure(_) => todo!(),
        }
    }
}
impl Add for StackVarType {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (StackVarType::Num(a), StackVarType::Num(b)) => StackVarType::Num(a + b),
            (StackVarType::Bool(a), StackVarType::Bool(b)) => StackVarType::Bool(a || b),
            _ => panic!("Type mismatch"),
        }
    }
}
impl Sub for StackVarType {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (StackVarType::Num(a), StackVarType::Num(b)) => StackVarType::Num(a - b),
            _ => panic!("Type mismatch"),
        }
    }
}
impl Mul for StackVarType {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (StackVarType::Num(a), StackVarType::Num(b)) => StackVarType::Num(a * b),
            (StackVarType::Bool(a), StackVarType::Bool(b)) => StackVarType::Bool(a && b),
            _ => panic!("Type mismatch"),
        }
    }
}
impl Div for StackVarType {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        match (self, other) {
            (StackVarType::Num(a), StackVarType::Num(b)) => StackVarType::Num(a / b),
            _ => panic!("Type mismatch"),
        }
    }
}