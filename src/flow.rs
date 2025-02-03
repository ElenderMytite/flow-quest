use std::cell::RefCell;

use crate::types::StackVarType;

#[derive(Debug, Clone)]
pub enum FlowListener{
    Console,
    Asserter(RefCell<Vec<StackVarType>>)
}
impl FlowListener {
    pub fn get(&self,val: StackVarType) -> bool {
        match self {
            FlowListener::Console => {
                match val {
                    StackVarType::Num(val) => println!("{}",val),
                    StackVarType::Bool(val) => println!("{}",val),
                    StackVarType::Tuple(vec) => println!("{:?}",vec),
                    StackVarType::Procedure(_) => println!("Procedure"),                 
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
    pub fn send(&self) -> StackVarType{
        match self {
            FlowStreamer::Console => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                StackVarType::Num(input.parse().unwrap())
            }
        }
    }
}