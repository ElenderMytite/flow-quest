use crate::old_runtime::VariableType;
use crate::types::ModulePath;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    pub vars: HashMap<String, VariableType>,
    pub packs: HashMap<String, VariableType>,
}
impl Environment {
    pub fn new(vars: HashMap<String, VariableType>, packs: HashMap<String, VariableType>) -> Self {
        Self { vars, packs }
    }
    pub fn construct_element(&mut self, name: Box<ModulePath>, value: VariableType, is_pack: bool) {
        match &name.child {
            Some(path) => {
                if self.vars.contains_key(&name.value) {
                    match &self.vars[&name.value] {
                        VariableType::Object(environment) => {
                            let mut next = environment.clone();
                            next.construct_element(name.child.unwrap(), value, is_pack);
                            if is_pack {
                                self.packs
                                    .insert(name.value.clone(), VariableType::Object(next));
                            } else {
                                self.vars.insert(name.value, VariableType::Object(next));
                            }
                        }
                        _ => panic!("Trying to insert to non object var other var!"),
                    }
                } else {
                    let mut child = Environment::new(HashMap::new(), HashMap::new());
                    child.construct_element(path.clone(), value, is_pack);
                    self.vars
                        .insert(name.value.clone(), VariableType::Object(child));
                }
            }
            None => {
                self.vars.insert(name.value.clone(), value);
            }
        }
    }
    pub fn assign_var(&mut self, name: Box<ModulePath>, value: VariableType) {
        match &name.child {
            Some(_) => {
                if self.vars.contains_key(&name.value) {
                    match &self.vars[&name.value] {
                        VariableType::Object(environment) => {
                            let mut next = environment.clone();
                            next.assign_var(name.child.unwrap(), value);
                            self.vars
                                .insert(name.value.clone(), VariableType::Object(next));
                        }
                        _ => panic!("Trying to insert to non object var other var!"),
                    }
                } else {
                    panic!("cannot find variable in current scope: {:?}", name);
                }
            }
            None => {
                if self.vars.contains_key(&name.value) {
                    self.vars.insert(name.value.clone(), value);
                } else {
                    panic!("cannot find variable in current scope: {:?}", name);
                }
            }
        }
    }
    pub fn look_up_element(&self, name: Box<ModulePath>) -> VariableType {
        match name.child {
            Some(_) => {
                if self.vars.contains_key(&name.value) {
                    match self.vars[&name.value].clone() {
                        VariableType::Object(environment) => environment.look_up_element(name),
                        v => v,
                    }
                } else if self.packs.contains_key(&name.value) {
                    match self.packs[&name.value].clone() {
                        VariableType::Object(environment) => environment.look_up_element(name),
                        v => v,
                    }
                } else {
                    panic!("cannot find variable in current scope: {:?}", name);
                }
            },
            None => {
                if self.vars.contains_key(&name.value) {
                    self.vars[&name.value].clone()
                } else if self.packs.contains_key(&name.value) {
                    self.packs[&name.value].clone()
                } else if &name.value == "-all" {
                    VariableType::Object(self.clone())
                } else {
                    println!("{:?}", self);
                    panic!("cannot find variable in current scope")
                }
            }
        }
    }
}
