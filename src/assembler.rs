use crate::types::StatementV;

pub fn create_asm(ast: &StatementV) -> String
{
    let mut asm_code = String::new();
    
    // Generate assembly code based on the AST
    match ast {
        StatementV::Block(statements, _) => {
                        for stmt in statements{
                            asm_code.push_str(&create_asm(&stmt.get_ast()));
                        }
            },
        StatementV::Nil => (),
        StatementV::Name(name) => {
            asm_code.push_str(&format!
                ("mov {}, {}\n", name, "value_placeholder")); // Placeholder for value
        },
        StatementV::Comparsion(comparsion_v, statement, statement1) => {
            let op = match comparsion_v {
                crate::types::ComparsionV::Equal => "je",
                crate::types::ComparsionV::Less => "jl",
                crate::types::ComparsionV::Greater => "jg",
                crate::types::ComparsionV::NotEqual => "jne",
                crate::types::ComparsionV::LessOrEqual => "jle",
                crate::types::ComparsionV::GreaterOrEqual => "jge",
            };
            asm_code.push_str(&format!(
                "cmp {}, {}\n{} label_placeholder\n", // Placeholder for label
                create_asm(&statement.get_ast()),
                create_asm(&statement1.get_ast()),
                op
            ));
        }
        StatementV::OperationBool(action_v, statement, statement1) => {
            let op = match action_v {
                crate::types::ActionV::Not => "not",
                crate::types::ActionV::And => "and",
                crate::types::ActionV::Or => "or",
                _ => todo!(), // Handle other operations
            };
            if op == "not" {
                asm_code.push_str(&format!(
                    "{} {}\n", 
                    op, 
                    create_asm(&statement.get_ast()),
                ));
            } else {
                asm_code.push_str(&format!(
                    "{} {}, {}\n", 
                    op, 
                    create_asm(&statement.get_ast()),
                    create_asm(&statement1.as_ref().unwrap().get_ast()),
                ));
            }
        },
        StatementV::OperationNumder(action_v, statement, statement1) => 
        {
            let op = match action_v {
                crate::types::ActionV::Add => "add",
                crate::types::ActionV::Sub => "sub",
                crate::types::ActionV::Div => "div",
                crate::types::ActionV::Mul => "mul",
                crate::types::ActionV::Mod => "mod",
                _ => todo!(), // Handle other operations
            };
            asm_code.push_str(&format!(
                "{} {}, {}\n", 
                op, 
                create_asm(&statement.get_ast()),
                create_asm(&statement1.get_ast()),
            ));

        }
        StatementV::If(statement, statement1, statement2) => {
                        asm_code.push_str(&create_asm(&statement.get_ast()));
                        asm_code.push_str(&create_asm(&statement1.get_ast()));
                        if let Some(else_stmt) = statement2 {
                            asm_code.push_str(&create_asm(&else_stmt.get_ast()));
                        }
            },
        _ => todo!()
    }
    asm_code
}
