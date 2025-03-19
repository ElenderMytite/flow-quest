use crate::types::{ActionV, BlockV, ComparsionV, Statement, StatementV, VarV};
use crate::types::{FlowListener, FlowStreamer};
use core::panic;
use std::cell::RefCell;
#[derive(Debug, Clone)]
#[allow(unused_variables, dead_code)]
pub enum MatchPattern {
    Var(String),
    Val(Vec<IR>),
    Unused,
}
#[allow(unused_variables, dead_code)]
#[derive(Debug, Clone)]
pub enum IR {
    Num(isize),
    Bool(bool),
    Code(Vec<IR>),

    Nil,

    BinExpr(ActionV),
    
    
    

    Not,
    Or,
    And,

    Eql,
    NEql,
    Ls,
    Gt,
    LsEql,
    GtEql,

    Store(String),
    Load(String),

    Jump(usize),

    Define(String, Vec<IR>),
    Exe(String),
    Efine(Vec<IR>),

    Input(RefCell<FlowStreamer>),
    Output(RefCell<FlowListener>),

    Case(Vec<MatchPattern>, usize),
}
pub fn ast_to_ir(ast_node: Statement, ir: &mut Vec<IR>) {
    match ast_node.get_ast() {
        StatementV::Block(vec, block_type) => match block_type {
            BlockV::Evaluate => {
                let mut ir_block: Vec<IR> = Vec::new();
                for node in vec {
                    ast_to_ir(node, &mut ir_block);
                }
                ir.push(IR::Efine(ir_block));
            }
            BlockV::Draft => {
                let mut ir_block: Vec<IR> = Vec::new();
                for node in vec {
                    ast_to_ir(node, &mut ir_block);
                }
                ir.push(IR::Code(ir_block));
            }
        },
        StatementV::Define { link, like } => {
            ast_to_ir(link, ir);
            ir.push(IR::Store(like));
        }
        StatementV::Call(transformer, statement) => {
            ast_to_ir(statement, ir);
            match transformer.get_ast() {
                StatementV::Name(name) => ir.push(IR::Exe(name)),
                value => ir.push(IR::Efine({let mut v =vec![]; ast_to_ir(Statement{value: Rc::new(value)},&mut v); v})),
                
            }
        }
        StatementV::Set { name, value } => {
            ast_to_ir(value, ir);
            ir.push(IR::Store(name));
        }
        StatementV::Nil => ir.push(IR::Nil),
        StatementV::Name(s) => {
            ir.push(IR::Load(s));
        }
        StatementV::Bool(v) => ir.push(IR::Bool(v)),
        StatementV::Number(v) => ir.push(IR::Num(v)),
        StatementV::Comparsion(comparsion_type, statement, statement1) => {
            ast_to_ir(statement, ir);
            ast_to_ir(statement1, ir);
            ir.push(match comparsion_type {
                ComparsionV::Equal => IR::Eql,
                ComparsionV::NotEqual => IR::NEql,
                ComparsionV::Less => IR::Ls,
                ComparsionV::Greater => IR::Gt,
                ComparsionV::LessOrEqual => IR::LsEql,
                ComparsionV::GreaterOrEqual => IR::GtEql,
            });
        }
        StatementV::OperationBool(action_type, statement, statement1) => {
            ast_to_ir(statement, ir);
            if let Some(statement1) = statement1 {
                ast_to_ir(statement1, ir);
            }
            ir.push(match action_type {
                ActionV::Not => IR::Not,
                ActionV::And => IR::And,
                ActionV::Or => IR::Or,
                _ => todo!(),
            });
        }
        StatementV::OperationNumder(action_type, statement, statement1) => {
            ast_to_ir(statement, ir);
            ast_to_ir(statement1, ir);
            ir.push(IR::BinExpr(action_type));
        }
        StatementV::If(statement, statement1, statement2) => {
            ast_to_ir(statement, ir);
            ir.push(IR::Case(
                vec![MatchPattern::Val(vec![IR::Bool(true)])],
                ir.len() + 2 + if statement2.is_some() { 1 } else { 0 },
            ));
            ast_to_ir(statement1, ir);
            if let Some(statement2) = statement2 {
                ir.push(IR::Jump(ir.len() + 2));
                ast_to_ir(statement2, ir);
            }
        }
        StatementV::Out { expr, to } => {
            ast_to_ir(expr, ir);
            ir.push(IR::Output(to));
        }
        StatementV::In(streamer) => ir.push(IR::Input(streamer)),
        StatementV::Jump(t) => ir.push(IR::Jump(if t { 0 } else { usize::MAX })),
    }
}
use std::collections::HashMap;
use std::rc::Rc;

pub fn execute(ir: Vec<IR>, heap: &mut HashMap<String, VarV>) -> Vec<VarV> {
    let mut stack: Vec<VarV> = Vec::new();
    let mut index = 0;
    while ir.len() > index {
        let instruction = ir.get(index).unwrap();
        match instruction {
            IR::Nil => (),
            IR::Num(n) => stack.push(VarV::Num(*n)),
            IR::Bool(b) => stack.push(VarV::Bool(*b)),
            IR::Code(c) => {
                stack.push(VarV::Procedure(c.clone()));
            }
            IR::BinExpr(_) |
            IR::Or |
            IR::And |
            IR::Not |
            IR::Eql |
            IR::NEql |
            IR::Ls |
            IR::Gt |
            IR::LsEql |
            IR::GtEql => {
                do_operation(&mut stack, instruction.clone());
            }
            IR::Store(name) => {
                let value = stack.pop().unwrap();
                heap.insert(name.clone(), value);
            }
            IR::Load(name) => {
                stack.push(heap[name].clone());
            }
            IR::Jump(jump_index) => {
                if *jump_index > ir.len() {
                    break;
                } else {
                    index = *jump_index;
                    continue;
                }
            }
            IR::Exe(name) => {
                stack.append(&mut execute(heap[name].get_code(), heap));
            }
            IR::Define(name, code) => {
                heap.insert(name.clone(), VarV::Procedure(code.clone()));
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
                let mut is_matching = true;
                for pat in pattern {
                    match pat {
                        MatchPattern::Var(name) => {
                            heap.insert(name.clone(), stack.pop().unwrap());
                        }
                        MatchPattern::Val(val) => {
                            if stack.pop().unwrap()
                                != return_vector_to_tuple(execute(val.clone(), heap))
                            {
                                is_matching = false;
                                break;
                            }
                        }
                        MatchPattern::Unused => {
                            stack.pop();
                        }
                    }
                }
                if !is_matching {
                    index = *gt;
                    continue;
                }
            }
            IR::Input(ref_cell) => {
                stack.push(ref_cell.borrow().send());
            }
            IR::Output(ref_cell) => {
                let top = stack.pop().unwrap();
                assert!(ref_cell.borrow().get(top.clone()));
            }
        }
        index += 1;
    }
    stack
}
fn return_vector_to_tuple(v: Vec<VarV>) -> VarV {
    match v.len() {
        0 => VarV::Tuple(Vec::new()),
        1 => v[0].clone(),
        _ => VarV::Tuple(v),
    }
}

fn do_operation(stack: &mut Vec<VarV>, operation: IR)
{
    let a = stack.pop().unwrap();
    if let IR::Not = operation {
        stack.push(!a);
        return;
    }
    let b = stack.pop().unwrap();
    stack.push(match operation {
        IR::BinExpr(action) => match action {
            ActionV::Add => b + a,
            ActionV::Sub => b - a,
            ActionV::Mul => b * a,
            ActionV::Div => b / a,
            _ => panic!("Unknown binary operation: {:?}", action),
        },
        IR::Or => b | a,
        IR::And => b & a,
        IR::Eql => VarV::Bool(a == b),
        IR::NEql => VarV::Bool(a != b),
        IR::Ls => VarV::Bool(b < a),
        IR::Gt => VarV::Bool(b > a),
        IR::LsEql => VarV::Bool(b <= a),
        IR::GtEql => VarV::Bool(b >= a),
        _ => panic!("Unknown binary operation: {:?}", operation),
    });
}
