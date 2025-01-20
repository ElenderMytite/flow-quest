use crate::environment::Environment;
use crate::types::{ActionType, ComparsionType, ExpressionType, Statement};
use std::{cell::RefCell, rc::Rc};
pub trait AlgebraicExpression {
    fn evaluate(&self, env: Rc<RefCell<Environment>>) -> isize;
}
pub trait BooleanExpression {
    fn evaluate_bool(&self, env: Rc<RefCell<Environment>>) -> bool;
}
impl AlgebraicExpression for ExpressionType {
    fn evaluate(&self, env: Rc<RefCell<Environment>>) -> isize {
        match self {
            ExpressionType::Number(v) => *v,
            ExpressionType::Name(path) => env
                .borrow()
                .look_up_element(path.clone())
                .return_variable_int(/*Rc::clone(&env)*/),
            ExpressionType::OperationNumder(action, left, right) => match action {
                ActionType::Plus => {
                    left.value.evaluate(Rc::clone(&env)) + right.value.evaluate(Rc::clone(&env))
                }
                ActionType::Minus => {
                    left.value.evaluate(Rc::clone(&env)) - right.value.evaluate(Rc::clone(&env))
                }
                ActionType::Divide => {
                    left.value.evaluate(Rc::clone(&env)) / right.value.evaluate(Rc::clone(&env))
                }
                ActionType::Multiply => {
                    left.value.evaluate(Rc::clone(&env)) * right.value.evaluate(Rc::clone(&env))
                }
                _ => panic!("expected int operation; not bool"),
            },
            ExpressionType::Bool(_) => panic!("trying to evaluate bool while evaluating int"),
            ExpressionType::If(condicition, if_, else_) => {
                if condicition.value.evaluate_bool(Rc::clone(&env)) {
                    if_.value.evaluate(Rc::clone(&env))
                } else {
                    match else_ {
                        Some(v) => v.value.evaluate(Rc::clone(&env)),
                        None => 0,
                    }
                }
            }
            ExpressionType::Comparsion(_, _, _) => {
                panic!("trying to evaluate bool while evaluating int")
            }
            ExpressionType::Nil => todo!(),
            ExpressionType::Block(stmts, _) => {
                if stmts.len() == 1 {
                    stmts[0].value.evaluate(Rc::clone(&env))
                } else {
                    panic!("program with two or more statements cannot be evaluated yet; ret keyword is still in development")
                }
            }
            ExpressionType::OutExpr { expr, like: _ } => expr.value.evaluate(Rc::clone(&env)),
            _ => panic!("cannot get int from {:?}", self),
        }
    }
}
impl BooleanExpression for ExpressionType {
    fn evaluate_bool(&self, env: Rc<RefCell<Environment>>) -> bool {
        match self {
            ExpressionType::Bool(v) => *v,
            ExpressionType::Name(path) => env
                .borrow_mut()
                .look_up_element(path.clone())
                .return_variable_bool(/*env.clone()*/),

            ExpressionType::Number(v) => *v != 0,

            ExpressionType::OperationBool(action, left, right) => match action {
                ActionType::And => {
                    left.value.evaluate_bool(Rc::clone(&env))
                        & right.clone().unwrap().value.evaluate_bool(Rc::clone(&env))
                }
                ActionType::Or => {
                    left.value.evaluate_bool(Rc::clone(&env))
                        | right.clone().unwrap().value.evaluate_bool(Rc::clone(&env))
                }
                ActionType::Not => !left.value.evaluate_bool(Rc::clone(&env)),
                _ => panic!(
                    "invalid bool action type: {:?}; left: {:?}; right: {:?};",
                    action, left, right
                ),
            },
            ExpressionType::Comparsion(type_, left, right) => match type_ {
                ComparsionType::Equal => {
                    left.value.evaluate(Rc::clone(&env)) == right.value.evaluate(Rc::clone(&env))
                }
                ComparsionType::Less => {
                    left.value.evaluate(Rc::clone(&env)) < right.value.evaluate(Rc::clone(&env))
                }
                ComparsionType::Greater => {
                    left.value.evaluate(Rc::clone(&env)) > right.value.evaluate(Rc::clone(&env))
                }
                ComparsionType::NotEqual => {
                    left.value.evaluate(Rc::clone(&env)) != right.value.evaluate(Rc::clone(&env))
                }
                ComparsionType::LessOrEqual => {
                    left.value.evaluate(Rc::clone(&env)) <= right.value.evaluate(Rc::clone(&env))
                }
                ComparsionType::GreaterOrEqual => {
                    left.value.evaluate(Rc::clone(&env)) >= right.value.evaluate(Rc::clone(&env))
                }
            },
            ExpressionType::If(condicition, if_, else_) => {
                if condicition.value.evaluate_bool(Rc::clone(&env)) {
                    if_.value.evaluate_bool(Rc::clone(&env))
                } else {
                    match else_ {
                        Some(v) => v.value.evaluate_bool(Rc::clone(&env)),
                        None => false,
                    }
                }
            }
            ExpressionType::Block(stmts, _block_type) => {
                if stmts.len() == 1 {
                    stmts[0].value.evaluate_bool(Rc::clone(&env))
                } else {
                    panic!("program with two or more statements cannot be evaluated yet; it is still in development")
                }
            }
            _ => panic!("cannot get bool from: {:?}", self),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum VariableType {
    Bool(bool),
    Int(isize),
    #[allow(dead_code)]
    Node(Rc<ExpressionType>),
    Object(Environment),
    Colletion(Vec<VariableType>),
}
pub fn run_statement(node: Statement, env: Rc<RefCell<Environment>>) -> Vec<VariableType> {
    let mut outs = vec![];
    // println!("running node: {:#?}", node);
    // let bind = node.clone();
    match &*node.value {
        ExpressionType::Block(stmts, _) => {
            for stmt in stmts {
                // println!("stmt: {:?}", stmt);
                match stmt.value.as_ref() {
                    ExpressionType::OutExpr { expr, like: _ } => {
                        let bind = Statement{value: expr.value.clone()};
                        outs.push(get_evaluation(bind, Rc::clone(&env)))
                    }
                    ExpressionType::Jump(again) => {
                        println!("jumping: {:?}", again);
                        if *again {
                            outs.append(&mut run_statement(
                                node.clone(),
                                Rc::clone(&env),
                            ));
                            break;
                        } else {
                            println!("jumping out of block");
                            break;
                        }
                    }
                    _ => {
                        println!("running stmt: {:?}", stmt);
                        outs.append(&mut run_statement(
                            stmt.clone(),
                            Rc::clone(&env),
                        ));
                    }
                }
            }
        }
        ExpressionType::Assign(name, value) => {
            let value = get_evaluation(value.clone(), Rc::clone(&env));
            env.borrow_mut().assign_var(name.clone(), value);
        }
        ExpressionType::Define { value, like } => {
            let value = get_evaluation(value.clone(), Rc::clone(&env));
            env.borrow_mut()
                .construct_element(like.clone(), value, false);
        }
        ExpressionType::Nil
        | ExpressionType::Bool(_)
        | ExpressionType::Number(_)
        | ExpressionType::Comparsion(_, _, _)
        | ExpressionType::Name(_)
        | ExpressionType::OperationBool(_, _, _)
        | ExpressionType::OperationNumder(_, _, _) => (),
        ExpressionType::If(cond, if_, else_) => {
            if cond.value.evaluate_bool(Rc::clone(&env)) {
                outs.append(&mut run_statement(if_.clone(), env));
            } else {
                match else_ {
                    Some(expr) => {
                        outs.append(&mut run_statement(expr.clone(), env));
                    }
                    None => (),
                }
            }
        }
        ExpressionType::OutExpr { expr, like: _ } => {
            outs.push(get_evaluation(expr.clone(), Rc::clone(&env)))
        }
        ExpressionType::In(_) => todo!(),
        _ => panic!("not yet implemented to run: {:?}", node),
    }
    // println!("run node {:#?};outs: {:?}",bind,outs);
    outs
}
fn get_evaluation(stmt: Statement, env: Rc<RefCell<Environment>>) -> VariableType {
    // println!("stmt: {:?}{:?}", stmt, env);
    let res = match stmt.value.as_ref() {
        ExpressionType::Bool(v) => VariableType::Bool(*v),
        ExpressionType::Number(v) => VariableType::Int(*v),
        ExpressionType::Comparsion(_, _, _) => {
            VariableType::Bool(stmt.value.evaluate_bool(Rc::clone(&env)))
        }
        ExpressionType::If(cond, left, right) => {
            if cond.value.evaluate_bool(Rc::clone(&env)) {
                get_evaluation(left.clone(), env)
            } else {
                let re = match right {
                    Some(v) => v,
                    None => &Statement::new(Rc::new(ExpressionType::Nil)),
                };
                get_evaluation(re.clone(), env)
            }
        }
        ExpressionType::OperationNumder(operand, _, _) => match operand {
            ActionType::Plus | ActionType::Minus | ActionType::Divide | ActionType::Multiply => {
                VariableType::Int(stmt.value.evaluate(Rc::clone(&env)))
            }
            _ => panic!("{:?} is not an int operation", operand),
        },
        ExpressionType::OperationBool(operand, _, _) => match operand {
            ActionType::And | ActionType::Or | ActionType::Not => {
                VariableType::Bool(stmt.value.evaluate_bool(Rc::clone(&env)))
            }
            _ => panic!("{:?} is not a bool operation", operand),
        },
        ExpressionType::Block(stmts, kind) => {
            if stmts.len() == 1 {
                println!("stmts: {:?}", stmts);
                get_evaluation(stmts[0].clone(), env)
            } else {
                match kind {
                    crate::types::BlockType::Draft => {
                        let mut collection: Vec<VariableType> = vec![];
                        for stmt in stmts {
                            collection.push(get_evaluation(stmt.clone(), env.clone()));
                        }
                        match collection.len() {
                            0 => VariableType::Colletion(vec![]),
                            1 => collection[0].clone(),
                            _ => VariableType::Colletion(collection),
                        }
                    }
                    crate::types::BlockType::Evaluate => {
                        if stmts.len() == 1 {
                            get_evaluation(stmts[0].clone(), env.clone())
                        } else {
                            let mut outs: Vec<VariableType> = vec![];
                            for statement in stmts {
                                match statement.value.as_ref() {
                                    ExpressionType::Jump(again) => {
                                        if *again {
                                            outs.append(&mut run_statement(
                                            stmt,
                                                Rc::clone(&env),
                                            ));
                                            break;
                                        } else {
                                            break;
                                        }
                                    }
                                    ExpressionType::OutExpr { expr, like: _ } => {
                                        outs.push(get_evaluation(expr.clone(), env.clone()))
                                    }
                                    v => {
                                        run_statement(
                                            Rc::new(v.clone()).into(),
                                            Rc::clone(&env),
                                        );
                                    }
                                }
                            }
                            VariableType::Colletion(outs)
                        }
                    }
                }
            }
        }
        ExpressionType::OutExpr { expr, like: _ } => get_evaluation(expr.clone(), env),
        ExpressionType::Name(path) => match &path.child {
            Some(child_path) => get_evaluation(
                Rc::new(ExpressionType::Name(child_path.clone())).into(),
                match env.borrow_mut().look_up_element(child_path.clone()) {
                    VariableType::Bool(_)
                    | VariableType::Int(_)
                    | VariableType::Node(_)
                    | VariableType::Colletion(_) => {
                        panic!("found non object with a child environment")
                    }
                    VariableType::Object(environment) => Rc::new(RefCell::new(environment)),
                },
            ),
            None => env.borrow_mut().look_up_element(path.clone()),
        },
        v => panic!("cant evaluate this Expr:  {:?} {:?}", stmt, v),
    };
    println!("res: {:?}", res);
    res
}
impl VariableType {
    pub fn return_variable_int(self, /*env: Rc<RefCell<Environment>> */) -> isize {
        match self {
            VariableType::Int(v) => v,
            _ => panic!("not an int!{:?}", self),
        }
    }
    pub fn return_variable_bool(self, /*env: Rc<RefCell<Environment>>*/) -> bool {
        match self {
            VariableType::Bool(v) => v,
            _ => panic!("not a bool!{:?}", self),
        }
    }
    pub fn print_self_value(self) {
        match self {
            VariableType::Bool(v) => print!("{v}"),
            VariableType::Int(v) => print!("{v}"),
            VariableType::Node(_) => print!("code"),
            VariableType::Object(_) => print!("object"),
            VariableType::Colletion(v) => print!("{:?}", v),
        }
    }
}
