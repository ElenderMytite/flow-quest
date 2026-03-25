use std::collections::HashMap;

use crate::ir::{MatchPattern, IR};
use crate::types::{ActionV, VarV};

pub fn execute(ir: Vec<IR>, heap: &mut HashMap<usize, VarV>) -> VarV {
    let mut stack: Vec<VarV> = Vec::new();
    let mut index = 0;
    while ir.len() > index {
        let instruction = ir.get(index).unwrap();
        match instruction {
            IR::Nil => (),
            IR::Num(n) => stack.push(VarV::Num(*n)),
            IR::Bool(b) => stack.push(VarV::Bool(*b)),
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
                heap.insert(*name, value);
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
            IR::Efine(vec) => {
                stack.append(&mut unpack(execute(vec.clone(), heap)));
            }
            IR::Case(patterns, gt) => {
                if patterns.len() > stack.len() || gt > &ir.len() {
                    panic!(
                        "Pattern length is longer than stack length or goto index is out of range"
                    );
                }
                let mut is_matching = true;
                for pattern in patterns {
                    match pattern {
                        MatchPattern::Var(name) => {
                            heap.insert(*name, stack.pop().unwrap());
                        }
                        MatchPattern::Val(val) => {
                            if stack.pop().unwrap()
                                != pack(unpack(execute(val.clone(), heap)))
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
                if is_matching {
                    index = *gt;
                    continue;
                }
            }
            IR::Input(streamer) => {
                stack.push(streamer.borrow().send());
            }
            IR::Output(listener) => {
                let top = stack.pop().unwrap();
                assert!(listener.borrow().get(top));
            }
        }
        index += 1;
    }
    pack(stack)
}
fn pack(v: Vec<VarV>) -> VarV {
    match v.len() {
        0 => VarV::Tuple(Vec::new()),
        1 => v[0].clone(),
        _ => VarV::Tuple(v),
    }
}
fn unpack(v: VarV) -> Vec<VarV> {
    match v {
        VarV::Tuple(vec) => vec,
        _ => vec![v],
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