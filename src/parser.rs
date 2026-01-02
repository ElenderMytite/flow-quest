use crate::types::{ActionV,Statement, Token};
use crate::types::{FlowListener, FlowStreamer};
use core::panic;
use std::cell::RefCell;
pub fn parse_program(tokens: &Vec<Token>, listener: &RefCell<FlowListener>) -> Statement {
    println!("parsing program...");
    println!("tokens: {:?}", tokens);
    parse_block(tokens, &mut 0, Token::EOF, listener)
}
pub fn parse_block(
    tokens: &Vec<Token>,
    index: &mut usize,
    closing_brace: Token,
    listener: &RefCell<FlowListener>,
) -> Statement {
    let mut statements: Vec<Box<Statement>> = Vec::new();
    while tokens.len() > *index && tokens[*index] != closing_brace && tokens[*index] != Token::EOF {
        let stmt: Statement = parse_statement(tokens, index, listener);
        statements.push(Box::from(stmt));
    }
    if tokens.len() == *index || tokens[*index] != closing_brace {
        panic!("expected closing brace: {:?}, found: {:?}", closing_brace, tokens.get(*index));
    }
    *index += 1;
    Statement::Block(statements)
}
fn parse_statement(tokens: &Vec<Token>, index: &mut usize, listener: &RefCell<FlowListener>) -> Statement {
    *index += 1;
    match tokens[*index - 1] {
        Token::Mark(id) => match id {
            8 => parse_if_statement(tokens, index, listener),
            3 => {
                let name = tokens[*index].name_id();
                *index += 1;
                let value: Box<Statement> = Box::from(parse_expression(tokens, index,1,listener));
                Statement::Set{ name, value }
            }
            16 => {
                let repeat = match tokens[*index] {
                    Token::Mark(17) => true,
                    Token::Mark(18) => false,
                    _ => panic!("expected again or stop"),
                };
                Statement::Jump(repeat)
            }
            19 => Statement::In(RefCell::new(FlowStreamer::None)),
            20 => {
                let to_out: Statement = parse_statement(tokens,index,  listener);
                Statement::Out {
                    expr: Box::from(to_out),
                    to: listener.clone(),
                }
            }
            _ => {
                parse_expression(tokens, index, 1, listener)
            }
        },
        _ => {
            *index -= 1;
            parse_expression(tokens, index,1, listener)
        }
    }
}
fn parse_if_statement(
    tokens: &Vec<Token>,
    index: &mut usize,
    listener: &RefCell<FlowListener>,
) -> Statement {
    let condition = parse_statement(tokens, index, listener);

    let if_block = parse_statement(tokens, index, listener);
    if let Token::Mark(12) = tokens[*index] {
        *index += 1;
        let else_block = parse_statement(tokens, index,listener);
        Statement::If(Box::from(condition), Box::from(if_block), Some(Box::from(else_block)))
    } else {
        Statement::If(Box::from(condition), Box::from(if_block), None)
    }
}
fn parse_expression(
    tokens: &Vec<Token>,
    index: &mut usize,
    min_priority: u8,
    listener: &RefCell<FlowListener>,
) -> Statement {
    if tokens.len() <= *index {
        return Statement::Nil;
    }
    let mut left_expr: Statement = parse_primary(tokens, index, listener);
    loop {
        let op: Token = tokens[*index];
        let priority = op.get_operation_priorety();
        if priority < min_priority || !op.is_operation() {
            break;
        }
        *index += 1;
        let right_expr: Statement = parse_expression(tokens, index,priority + 1, listener);
        left_expr = match &op {
            Token::Sign(_) => Statement::OperationNumder(
                op.token_to_action_type(),
                Box::from(left_expr),
                Box::from(right_expr),
            )
            .into(),
            Token::Comparsion(_) => Statement::Comparsion(
                op.token_to_comparsion_type(),
                Box::from(left_expr),
                Box::from(right_expr),
            )
            .into(),
            Token::Mark(1 | 7 | 9) => Statement::OperationBool(
                op.token_to_action_type(),
                Box::from(left_expr),
                Some(Box::from(right_expr)),
            ),
            _ => panic!("Invalid operation to operate"),
        };
    }
    left_expr
}
fn parse_primary<'a>(tokens: &Vec<Token>,index: &mut usize, listener: &RefCell<FlowListener>) -> Statement {
    if tokens.len() <= *index {
        return Statement::Nil;
    }
    let tk: Token = tokens[*index];
    *index += 1;
    match tk {
        Token::Brackets { id, is_opened } => parse_brackets(tokens, index, id, is_opened, listener),
        Token::Mark(1) | Token::Sign(2) => {
            let expr: Statement = parse_expression(tokens, index, 5, listener);
            Statement::OperationBool(ActionV::Not, Box::from(expr), None)
        }
        Token::Number(val) => Statement::Number(val),
        Token::Bool(val) => Statement::Bool(val),
        Token::Name(name) => Statement::Name(name),
        _ => Statement::Nil,
    }
}
fn parse_brackets(
    tokens: &Vec<Token>,
    index: &mut usize,
    id: u8,
    is_opened: bool,
    listener: &RefCell<FlowListener>,
) -> Statement {
    if is_opened {
        let closing_brace = Token::Brackets { id, is_opened: false };
        let block = parse_block(tokens, index, closing_brace, listener);
        block
    } else {
        panic!("unexpected closing bracket");
    }
}
