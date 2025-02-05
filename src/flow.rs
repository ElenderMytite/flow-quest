use std::cell::RefCell;

use crate::types::VarT;

#[derive(Debug, Clone)]
pub enum FlowListener{
    Console,
    Asserter(RefCell<Vec<VarT>>)
}
impl FlowListener {
    pub fn get(&self,val: VarT) -> bool {
        match self {
            FlowListener::Console => {
                match val {
                    VarT::Num(val) => println!("{}",val),
                    VarT::Bool(val) => println!("{}",val),
                    VarT::Tuple(vec) => println!("{:?}",vec),
                    VarT::Procedure(_) => println!("Procedure"),                 
                }
                true
            }
            FlowListener::Asserter(expected_values) => {
                if expected_values.borrow_mut().remove(0) == val {
                    true
                } else {
                    false
                }
            }
        }
    }
}
#[derive(Debug, Clone)]
pub enum FlowStreamer{
    Console,
}
impl FlowStreamer {
    pub fn send(&self) -> VarT{
        match self {
            FlowStreamer::Console => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                VarT::Num(input.parse().unwrap())
            }
        }
    }
}