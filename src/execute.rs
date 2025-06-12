use std::collections::HashMap;

use crate::ir::{MatchPattern, IR};
use crate::types::{ActionV, VarV};

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
            ActionV::Mod => b % a,
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