use std::collections::HashMap;

use crate::{ir::IR, types::ActionV};
pub struct AssemblyGenerator {
    irs: Vec<IR>,
    asm: String,
    vars: HashMap<String, isize>,
    stack_size: isize,
}
impl AssemblyGenerator {
    pub fn new(irs: Vec<IR>, asm: String, vars: HashMap<String, isize>, stack_size: isize) -> Self {
        Self { irs, asm, vars, stack_size }
    }
    
    pub fn assembly(&mut self) {
        self.asm = "section .text\n\tglobal _start\n_start:\n".to_string();
        for ir_stmt in self.irs.clone() {
            AssemblyGenerator::gen_asm(self, ir_stmt.clone())
        }
        self.asm += "\tmov rax, 60\n\tmov rdi,0\n\tsyscall\n";
    }
    fn gen_asm(&mut self, ir: IR) {
        let s = &mut self.asm;
        match ir {
            IR::Num(i) => {
                *s += &format!("\tpush {}\n", i);
            }
            IR::Bool(b) => {
                *s += &format!("\tpush {}\n", if b { 1 } else { 0 });
            }
            IR::Code(_) => (),
            IR::Nil => (),
            IR::BinExpr(var) => {
                let op = match var {
                    ActionV::Add => "add",
                    ActionV::Sub => "sub",
                    ActionV::Mul => "mul",
                    ActionV::Div => "div",
                    _ => panic!("Unknown operator"),
                };
                *s += &format!("\tpop rax\n\t{} qword [rsp] rax, \n", op);
            }
            IR::Not => (),
            IR::Or => (),
            IR::And => (),
            IR::Eql => (),
            IR::NEql => (),
            IR::Ls => (),
            IR::Gt => (),
            IR::LsEql => (),
            IR::GtEql => (),
            IR::Store(val) => {
                self.vars.insert(val, self.stack_size);
                self.stack_size += 8;
            }
            IR::Load(val) => {
                *s += &format!("\tmov rax, [rsp+{}]\n\tpush rax\n", self.vars[&val]);
            }
            IR::Jump(_) => (),
            IR::Define(_, _) => (),
            IR::Exe(_) => (),
            IR::Efine(ir_stmts) => {
                for ir in ir_stmts {
                    AssemblyGenerator::gen_asm(self,ir.clone())
                }
            }
            IR::Input(_) => (),
            IR::Output(_) => (),
            IR::Case(_, _) => (),
        }
    }
    pub fn print_asm(&self) {
        println!("asm: \n\n{}", self.asm);
    }
    pub fn save_asm(&self, path: String) {
        crate::inout::create_file(path, self.asm.clone());
    }
}
