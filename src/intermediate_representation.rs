use crate::flow::{FlowListener, FlowStreamer};
use crate::types::{ActionType, BlockType, ComparsionType, ExpressionType, StackVarType, Statement};
use std::cell::RefCell;
#[derive(Debug, Clone)]
#[allow(unused_variables, dead_code)]
pub enum MatchPattern {
    Var(String),
    Val(Vec<IR>),
    Unused,
}
#[allow(unused)]
#[derive(Debug, Clone)]
pub enum IR {
    Num(isize),
    Bool(bool),
    Code(Vec<IR>),
    Nil,
    Add,
    Sub,
    Mul,
    Div,
    Not,

    Eql,
    NEql,
    Ls,
    Gt,
    LsEql,
    GtEql,

    Store(String),
    Load(String),

    Jump(bool),
    Define(String, Vec<IR>),
    Exe(String),
    Efine(Vec<IR>),

    Input(RefCell<FlowStreamer>),
    Output(RefCell<FlowListener>),

    Case(Vec<MatchPattern>, usize),

    Free(usize),
}
#[allow(dead_code, unused_variables)]
pub fn assembly(code: Vec<IR>) -> String {
    todo!()
}
pub fn ast_to_ir(ast_node: Statement, ir: &mut Vec<IR>,listener: &RefCell<FlowListener>) {
    match ast_node.get_ast() {
        ExpressionType::Block(vec, block_type) => match block_type {
            BlockType::Evaluate => {
                let mut ir_block: Vec<IR> = Vec::new();
                for node in vec {
                    ast_to_ir(node, &mut ir_block,listener);
                }
                ir.push(IR::Efine(ir_block));
            }
            BlockType::Draft => {
                let mut ir_block: Vec<IR> = Vec::new();
                for node in vec {
                    ast_to_ir(node, &mut ir_block,listener);
                }
                ir.push(IR::Code(ir_block));
            }
        },
        ExpressionType::Define { link, like } => {
            ast_to_ir(link, ir, listener);
            ir.push(IR::Store(like));
        }
        ExpressionType::Assign(module_path, statement) => {
            ast_to_ir(statement, ir, listener);
            ir.push(IR::Store(module_path));
        }
        ExpressionType::Set{ name, value} => {
            ast_to_ir(value, ir, listener);
            ir.push(IR::Store(name));
        }
        ExpressionType::Nil => ir.push(IR::Nil),
        ExpressionType::Name(s) => {
            ir.push(IR::Load(s));
        }
        ExpressionType::Bool(v) => ir.push(IR::Bool(v)),
        ExpressionType::Number(v) => ir.push(IR::Num(v)),
        ExpressionType::Comparsion(comparsion_type, statement, statement1) => {
            ast_to_ir(statement, ir, listener);
            ast_to_ir(statement1, ir, listener);
            ir.push(match comparsion_type {
                ComparsionType::Equal => IR::Eql,
                ComparsionType::NotEqual => IR::NEql,
                ComparsionType::Less => IR::Ls,
                ComparsionType::Greater => IR::Gt,
                ComparsionType::LessOrEqual => IR::LsEql,
                ComparsionType::GreaterOrEqual => IR::GtEql,
            });
        }
        ExpressionType::OperationBool(action_type, statement, statement1) => {
            ast_to_ir(statement, ir, listener);
            if let Some(statement1) = statement1 {
                ast_to_ir(statement1, ir, listener);
            }
            ir.push(match action_type {
                ActionType::Not => IR::Not,
                ActionType::And => IR::Add,
                ActionType::Or => IR::Mul,
                _ => todo!(),
            });
        }
        ExpressionType::OperationNumder(action_type, statement, statement1) => {
            ast_to_ir(statement, ir, listener);
            ast_to_ir(statement1, ir, listener);
            ir.push(match action_type {
                ActionType::Plus => IR::Add,
                ActionType::Minus => IR::Sub,
                ActionType::Divide => IR::Div,
                ActionType::Multiply => IR::Mul,
                _ => todo!(),
            });
        }
        ExpressionType::If(statement, statement1, statement2) => {
            ast_to_ir(statement, ir, listener);
            ir.push(IR::Case(
                vec![MatchPattern::Val(vec![IR::Bool(true)])],
                ir.len() + 2,
            ));
            ast_to_ir(statement1, ir, listener);
            if let Some(statement2) = statement2 {
                ast_to_ir(statement2, ir, listener);
            }
        }
        ExpressionType::OutExpr { expr, like: _ } => {
            ast_to_ir(expr, ir, listener);
            ir.push(IR::Output(RefCell::new(FlowListener::Console)));
        }
        ExpressionType::In(_) => ir.push(IR::Input(RefCell::new(FlowStreamer::Console))),
        ExpressionType::Jump(t) => ir.push(IR::Jump(t)),
    }
}
use std::collections::HashMap;
#[allow(dead_code)]
pub fn execute(
    ir: Vec<IR>,
    heap: &mut HashMap<String, StackVarType>,
) -> Vec<StackVarType> {
    let mut stack: Vec<StackVarType> = Vec::new();
    heap.insert(String::from("s-main"), StackVarType::Procedure(ir.clone()));
    let mut index = 0;
    while ir.len() > index {
        let instruction = ir.get(index).unwrap();
        match instruction {
            IR::Nil => (),
            IR::Num(n) => stack.push(StackVarType::Num(*n)),
            IR::Bool(b) => stack.push(StackVarType::Bool(*b)),
            IR::Code(c) => {
                stack.push(StackVarType::Procedure(c.clone()));
            }
            IR::Add => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b + a);
            }
            IR::Sub => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b - a);
            }
            IR::Mul => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b * a);
            }
            IR::Div => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(b / a);
            }
            IR::Not => {
                let a = stack.pop().unwrap();
                stack.push(!a);
            }
            IR::Eql => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(StackVarType::Bool(a == b));
            }
            IR::NEql => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(StackVarType::Bool(a != b));
            }
            IR::Ls => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(StackVarType::Bool(a > b));
            }
            IR::Gt => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(StackVarType::Bool(a < b));
            }
            IR::LsEql => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(StackVarType::Bool(a >= b));
            }
            IR::GtEql => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(StackVarType::Bool(a <= b));
            }
            IR::Store(name) => {
                let value = stack.pop().unwrap();
                heap.insert(name.clone(), value);
            }
            IR::Load(name) => {
                println!("load: {}; heap: {:?}", name, heap);
                stack.push(heap[name].clone());
            }
            IR::Jump(again) => {
                if *again {
                    index = 0;
                    continue;
                }
                break;
            }
            IR::Exe(name) => {
                stack.append(&mut execute(heap[name].get_code(), heap));
            }
            IR::Define(name, code) => {
                heap.insert(name.clone(), StackVarType::Procedure(code.clone()));
            }
            IR::Efine(vec) => {
                stack.append(&mut execute(vec.clone(), heap));
            }
            IR::Case(pattern, gt) => {
                if pattern.len() > stack.len() || gt > &ir.len() {
                    panic!(
                        "Pattern length is longer than stack length or goto index is out of range"
                    );
                }
                let mut matched = true;
                for pat in pattern {

                    match pat {
                        MatchPattern::Var(name) => {
                            heap.insert(name.clone(), stack.last().unwrap().clone());
                        }
                        MatchPattern::Val(val) => {
                            if stack.pop().unwrap()
                            != return_vector_to_tuple(execute(val.clone(), heap))
                            {
                                matched = false;
                                break;
                            }
                        }
                        MatchPattern::Unused => {
                            stack.pop();
                        }
                    }
                }
                if !matched {
                    index = *gt;
                    continue;
                }
            }
            IR::Free(size) => {
                stack.truncate(stack.len() - size);
            }
            IR::Input(ref_cell) => {
                stack.push(ref_cell.borrow().send());
            }
            IR::Output(ref_cell) => {
                let ok_run = ref_cell.borrow().get(stack.pop().unwrap());
                assert!(ok_run);
            }
        }
        index += 1;
    }
    stack
}
fn return_vector_to_tuple(v: Vec<StackVarType>) -> StackVarType {
    match v.len() {
        0 => StackVarType::Tuple(Vec::new()),
        1 => v[0].clone(),
        _ => StackVarType::Tuple(v),
    }
}
