use std::rc::Rc;

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

#[derive(Debug, Clone,PartialEq)]
#[allow(dead_code)]
pub enum ToInvoke {
    Named(String),
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
#[derive(Debug,Clone,PartialEq)]
#[allow(dead_code)]
pub enum ExpressionType {
    Block(Vec<Statement>,BlockType),
    Define{link: Statement,like: String},
    Assign(String, Statement),
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
    Set{name: String,value: Statement},
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