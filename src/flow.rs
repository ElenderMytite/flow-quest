use crate::intermediate_representation::StackVarType;

#[derive(Debug, Clone)]
pub enum FlowListener{
    Console,
}
impl FlowListener {
    pub fn get(&self,val: StackVarType) {
        match self {
            FlowListener::Console => {
                match val {
                    StackVarType::Num(val) => println!("{}",val),
                    StackVarType::Bool(val) => println!("{}",val),
                    StackVarType::Tuple(vec) => println!("{:?}",vec),
                    StackVarType::Procedure(_) => println!("Procedure"),                 
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