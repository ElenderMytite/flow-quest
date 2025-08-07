use crate::types::{ActionV, BlockV, ComparsionV, Statement, StatementV};
use crate::types::{FlowListener, FlowStreamer};
use core::panic;
use std::cell::RefCell;
use std::rc::Rc;
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
pub fn ast_to_ir(ast_node: Rc<StatementV>, ir: &mut Vec<IR>) {
    match ast_node.as_ref() {
        StatementV::Block(vec, block_type) => match block_type {
            BlockV::Evaluate => {
                let mut ir_block: Vec<IR> = Vec::new();
                for node in vec {
                    ast_to_ir(Rc::clone(&Rc::clone(&node.value)), &mut ir_block);
                }
                ir.push(IR::Efine(ir_block));
            }
            BlockV::Draft => {
                let mut ir_block: Vec<IR> = Vec::new();
                for node in vec {
                    ast_to_ir(Rc::clone(&node.value), &mut ir_block);
                }
                ir.push(IR::Code(ir_block));
            }
            _ => panic!("Block type {:?} not implemented yet", block_type),
        },
        StatementV::Define { link, like } => {
            ast_to_ir(Rc::clone(&link.value), ir);
            ir.push(IR::Store(like.clone()));
        }
        StatementV::Call(transformer, statement) => {
            ast_to_ir(Rc::clone(&statement.value), ir);
            match transformer.get_ast() {
                StatementV::Name(name) => ir.push(IR::Exe(name)),
                value => ir.push(IR::Efine({let mut v =vec![]; 
                ast_to_ir(Rc::new(value),&mut v); v})),
            }
        }
        StatementV::Set { name, value } => {
            ast_to_ir(Rc::clone(&value.value), ir);
            ir.push(IR::Store(name.clone()));
        }
        StatementV::Nil => ir.push(IR::Nil),
        StatementV::Name(s) => {
            ir.push(IR::Load(s.clone()));
        }
        StatementV::Bool(v) => ir.push(IR::Bool(*v)),
        StatementV::Number(v) => ir.push(IR::Num(*v)),
        StatementV::Comparsion(comparsion_type, statement, statement1) => {
            ast_to_ir(Rc::clone(&statement.value), ir);
            ast_to_ir(Rc::clone(&statement1.value), ir);
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
            ast_to_ir(Rc::clone(&statement.value), ir);
            if let Some(statement1) = statement1 {
                ast_to_ir(Rc::clone(&statement1.value), ir);
            }
            ir.push(match action_type {
                ActionV::Not => IR::Not,
                ActionV::And => IR::And,
                ActionV::Or => IR::Or,
                _ => todo!(),
            });
        }
        StatementV::OperationNumder(action_type, statement, statement1) => {
            ast_to_ir(Rc::clone(&statement.value), ir);
            ast_to_ir(Rc::clone(&statement1.value), ir);
            ir.push(IR::BinExpr(action_type.clone()));
        }
        StatementV::If(statement, statement1, statement2) => {
            ast_to_ir(Rc::clone(&statement.value), ir);
            ir.push(IR::Case(
                vec![MatchPattern::Val(vec![IR::Bool(true)])],
                ir.len() + 2 + if statement2.is_some() { 1 } else { 0 },
            ));
            ast_to_ir(Rc::clone(&statement1.value), ir);
            if let Some(statement2) = statement2 {
                ir.push(IR::Jump(ir.len() + 2));
                ast_to_ir(Rc::clone(&statement2.value), ir);
            }
        }
        StatementV::Out { expr, to } => {
            ast_to_ir(Rc::clone(&expr.value), ir);
            ir.push(IR::Output(to.clone()));
        }
        StatementV::In(streamer) => ir.push(IR::Input(streamer.clone())),
        StatementV::Jump(t) => ir.push(IR::Jump(if *t { 0 } else { usize::MAX })),
    }
}
#[allow(unused_variables, dead_code)]
pub fn generate_bytecode(ast_node: Statement) -> Vec<u8> {
    let mut code: Vec<u8> = Vec::new();
    match ast_node.get_ast() {
        StatementV::Block(statements, block_v) => todo!(),
        StatementV::Define { link, like } => todo!(),
        StatementV::Call(statement, statement1) => todo!(),
        StatementV::Set { name, value } => todo!(),
        StatementV::Nil => (),
        StatementV::Name(n) => todo!(),
        StatementV::Bool(b) => {
            code.push(0);
            code.push(if b { 1 } else { 0 });
        },
        StatementV::Number(n) => {
            code.push(0);
            let bytes = n.to_le_bytes();
            code.extend_from_slice(&bytes);
        },
        StatementV::Comparsion(comparsion_v, statement, statement1) => todo!(),
        _ => (),
    }
    code.push(255);
    code
}