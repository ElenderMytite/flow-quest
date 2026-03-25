use crate::types::{ActionV,ComparsionV, Statement};
use crate::types::{FlowListener, FlowStreamer};
use std::cell::RefCell;
#[derive(Debug, Clone)]
#[allow(unused_variables, dead_code)]
pub enum MatchPattern {
    Var(usize),
    Val(Vec<IR>),
    Unused,
}
#[derive(Debug, Clone)]
pub enum IR {
    Num(isize),
    Bool(bool),

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

    Store(usize),
    Load(usize),

    Jump(usize),
    Efine(Vec<IR>),

    Input(RefCell<FlowStreamer>),
    Output(RefCell<FlowListener>),

    Case(Vec<MatchPattern>, usize),
}
pub fn ast_to_ir(ast_node: &Statement, ir: &mut Vec<IR>) {
    match ast_node {
        Statement::Block(vec) =>{
            let mut ir_block: Vec<IR> = Vec::new();
            for node in vec {
                ast_to_ir(&node, &mut ir_block);
            }
            ir.push(IR::Efine(ir_block));
        },
        
        Statement::Set { name, value } => {
            ast_to_ir(value, ir);
            ir.push(IR::Store(*name));
        }
        Statement::Nil => ir.push(IR::Nil),
        Statement::Name(s) => {
            ir.push(IR::Load(s.clone()));
        }
        Statement::Bool(v) => ir.push(IR::Bool(*v)),
        Statement::Number(v) => ir.push(IR::Num(*v)),
        Statement::Comparsion(comparsion_type, statement, statement1) => {
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
        Statement::OperationBool(action_type, statement, statement1) => {
            ast_to_ir(&statement, ir);
            if let Some(statement1) = statement1 {
                ast_to_ir(&statement1, ir);
            }
            ir.push(match action_type {
                ActionV::Not => IR::Not,
                ActionV::And => IR::And,
                ActionV::Or => IR::Or,
                _ => {println!("Invalid boolean operation in AST to IR conversion"); IR::Nil},
            });
        }
        
        Statement::OperationNumder(action_type, statement, statement1) => {
            ast_to_ir(&statement, ir);
            ast_to_ir(&statement1, ir);
            ir.push(IR::BinExpr(action_type.clone()));
        }
        Statement::If(statement, statement1, statement2) => {
            ast_to_ir(&statement, ir);
            ir.push(IR::Case(
                vec![MatchPattern::Val(vec![IR::Bool(false)])],
                ir.len() + 2 + if statement2.is_some() { 1 } else { 0 }, // jump over the "then" block and optional "else" block
            ));
            ast_to_ir(&statement1, ir);
            if let Some(statement2) = statement2 {
                ir.push(IR::Jump(ir.len() + 2)); // jump over the "else" block
                ast_to_ir(&statement2, ir);
            }
        }
        Statement::Out { expr, to } => {
            ast_to_ir(&expr, ir);
            ir.push(IR::Output(to.clone()));
        }
        Statement::In(streamer) => ir.push(IR::Input(streamer.clone())),
        Statement::Jump(t) => ir.push(IR::Jump(if *t { 0 } else { usize::MAX })),
    }
}