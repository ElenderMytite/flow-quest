use std::{borrow::Borrow,rc::Rc,};

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
pub type Path = Option<Box<ModulePath>>;

#[derive(Debug, Clone,PartialEq)]
#[allow(dead_code)]
pub enum ToInvoke {
    Named(Box<ModulePath>),
    Unnamed { code: Statement },
}
impl ToInvoke {
    pub fn get_ast(&self) -> Statement {
        match self {
            ToInvoke::Named(module_path) => {
                Statement::new(Rc::new(ExpressionType::Name(module_path.clone())))
            }
            ToInvoke::Unnamed { code } => code.clone(),
        }
    }
}
#[derive(Debug, Clone,PartialEq)]
#[allow(dead_code)]
pub struct ModulePath {
    pub value: String,
    pub child: Option<Box<ModulePath>>,
}
impl ModulePath {
    pub fn new(value: String, child: Option<Box<ModulePath>>) -> Self {
        Self { value, child }
    }
    pub fn get_string(&self) -> String {
        match &self.child {
            Some(child) => {
                let bind: &ModulePath = child.borrow();
                format!("{}_{}", self.value, bind.get_string())
            }
            None => self.value.clone(),
        }
    }
    
}
pub fn create_module_path(nodes: Vec<String>) -> Option<Box<ModulePath>> {
    if nodes.len() == 0 {
        return None;
    }
    Some(Box::new(ModulePath {
        value: nodes[0].clone(),
        child: create_module_path(nodes[1..].to_vec()),
    }))
}
#[derive(Debug,Clone,PartialEq)]
#[allow(dead_code)]
pub enum ExpressionType {
    Block(Vec<Statement>,BlockType),
    Define{value: Statement,like: Box<ModulePath>},
    Assign(Box<ModulePath>, Statement),
    Nil,
    Name(Box<ModulePath>),
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
    // Flow,
    Evaluate,
    Draft,
}
#[derive(Debug,Clone,PartialEq)]
#[allow(dead_code)]
pub enum BraceType {
    Angle,
    Square,
    Curly,
}