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
# [allow(dead_code)]
pub fn execute_bytecode(bytecode: Vec<u8>) -> Stack {
    let mut ip: usize = 0;
    let mut stack: Stack = Stack::new();
    loop {
        if ip >= bytecode.len() {
            break;
        }
        let instruction = bytecode[ip];
        match instruction {
            0 => {
                // PUSH
                stack.push(&[bytecode.get(ip + 1).unwrap_or(&0).clone()]);
            }
            1 => {
                // POP
                stack.pop(1);
            }
            2 => {
                // ADD
                let a = stack.pop(1)[0];
                let b = stack.pop(1)[0];
                stack.push(&(b as u16 + a as u16).to_le_bytes());
            }
            3 => {
                // SUB
                let a = stack.pop(1)[0];
                let b = stack.pop(1)[0];
                stack.push(&(b as u16 - a as u16).to_le_bytes());
            }
            4 => {
                // MUL
                let a = stack.pop(1)[0];
                let b = stack.pop(1)[0];
                stack.push(&(b as u16 * a as u16).to_le_bytes());
            }
            5 => {
                // DIV                
                let a = stack.pop(1)[0];
                let b = stack.pop(1)[0];
                stack.push(&(b as u16 / a as u16).to_le_bytes());
            }
            6 => {
                // MOD
                let a = stack.pop(1)[0];
                let b = stack.pop(1)[0];
                stack.push(&[b % a]);
            }
            7 => {
                // EQL
                let a = stack.pop(1)[0];
                let b = stack.pop(1)[0];
                stack.push(&[if b == a { 1 } else { 0 }]);
            }
            8 => {
                // NEQL
                let a = stack.pop(1)[0];
                let b = stack.pop(1)[0];
                stack.push(&[if b != a { 1 } else { 0 }]);            
            }
            9 => {
                // LS
                let a = stack.pop(1)[0];
                let b = stack.pop(1)[0];
                stack.push(&[if b < a { 1 } else { 0 }]);
            }
            10 => {
                // GT
                let a = stack.pop(1)[0];
                let b = stack.pop(1)[0];
                stack.push(&[if b > a { 1 } else { 0 }]);
            }
            11 => {
                // LSEQL
                let a = stack.pop(1)[0];
                let b = stack.pop(1)[0];
                stack.push(&[if b <= a { 1 } else { 0 }]);
            }
            12 => {
                // GTEQL
                let a = stack.pop(1)[0];
                let b = stack.pop(1)[0];
                stack.push(&[if b >= a { 1 } else { 0 }]);
            }
            13 => {
                // NOT
                let a = stack.pop(1)[0];
                stack.push(&[if a == 0 { 1 } else { 0 }]);
            }
            14 => {
                // AND
                let a = stack.pop(1)[0];
                let b = stack.pop(1)[0];
                stack.push(&[if b != 0 && a != 0 { 1 } else { 0 }]);
            }
            15 => {
                // OR
                let a = stack.pop(1)[0];
                let b = stack.pop(1)[0];
                stack.push(&[if b != 0 || a != 0 { 1 } else { 0 }]);
            }
            16 => {
                // XOR
                let a = stack.pop(1)[0];
                let b = stack.pop(1)[0];
                stack.push(&[if (b != 0) ^ (a != 0) { 1 } else { 0 }]);
            }
            17 => {
                // BIT_AND
                let a = stack.pop(1)[0];
                let b = stack.pop(1)[0];
                stack.push(&[b & a]);
            }
            18 => {
                // BIT_OR
                let a = stack.pop(1)[0];
                let b = stack.pop(1)[0];
                stack.push(&[b | a]);
            }
            19 => {
                // BIT_XOR
                let a = stack.pop(1)[0];
                let b = stack.pop(1)[0];
                stack.push(&[b ^ a]);
            }
            20 => {
                // BIT_NOT
                let a = stack.pop(1)[0];
                stack.push(&[!a]);
            }
            21 => {
                // DUP
                let a = stack.get(stack.len() - 1);
                stack.push(&[a]);
            }
            22 => {
                // SWAP
                let a = stack.pop(1)[0];
                let b = stack.pop(1)[0];
                stack.push(&[a]);
                stack.push(&[b]);
            }
            // 127 => {
            //     // LOAD
            //     let address = bytecode.get(ip + 1).unwrap_or(&0).clone() as usize;
            //     if address < stack.len() {
            //         stack.push(stack.data[address]);
            //     } else {
            //         panic!("Load instruction out of bounds");
            //     }
            // }
            // 128 => {
            //     // STORE
            //     let address = bytecode.get(ip + 1).unwrap_or(&0).clone() as usize;
            //     if address < stack.len() {
            //         stack.data[address] = stack.pop(1)[0];
            //     } else {
            //         panic!("Store instruction out of bounds");
            //     }
            // }
            129 => {
                // JUMP
                let jump_address = bytecode.get(ip + 1).unwrap_or(&0).clone() as usize;
                if jump_address < bytecode.len() {
                    ip = jump_address;
                    continue; 
                } else {
                    panic!("Jump instruction out of bounds");
                }
            }
            130 => {
                // JUMP_IF_TRUE
                let jump_address = bytecode.get(ip + 1).unwrap_or(&0).clone() as usize;
                let condition = stack.pop(1)[0];
                if condition != 0 {
                    if jump_address < bytecode.len() {
                        ip = jump_address;
                        continue; 
                    } else {
                        panic!("Jump instruction out of bounds");
                    }
                }
            }
            254 => {
                // PRINT
                let value = stack.pop(1)[0];
                println!("{}", value);
            }
            255 =>{
                // HALT
                break;
            }
            _ => {
                println!("Unknown instruction: {}", instruction);
            }
        }
        ip += 1;
    }
    stack
}
pub struct Stack {
    data: [u8; STACK_SIZE],
    sp : usize,
}
impl Stack {
    pub fn new() -> Self {
        Stack {
            data: [0; STACK_SIZE],
            sp: 0,
        }
    }
    fn push(&mut self, bytes: &[u8]) {
        if self.sp + bytes.len() > self.data.len() {
            panic!("Stack overflow: not enough space to push bytes");
        }
        self.data[self.sp..self.sp + bytes.len()].copy_from_slice(bytes);
        self.sp += bytes.len();
    }
    fn pop(&mut self, count: usize) -> &[u8]{
        if count > self.sp {
            panic!("Stack underflow: not enough bytes to pop");
        }
        let start = self.sp - count;
        let bytes = &self.data[start..self.sp];
        self.sp -= count; 
        bytes
    }
    pub fn get(&self, index: usize) -> u8 {
        if self.sp > index {
            self.data[index ]
        } else {
            panic!("Stack index out of bounds");
        }
    }
    pub fn len(&self) -> usize {
        self.sp
    }
}
const STACK_SIZE: usize = 1024;