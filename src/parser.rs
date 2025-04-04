use crate::types::Statement;
use crate::types::{ActionV, BlockV, StatementV, TokenV};
use crate::types::{FlowListener, FlowStreamer};
use core::panic;
use std::cell::RefCell;
use std::rc::Rc;

fn get_data_from_chain(
    tokens: &mut Vec<TokenV>,
    listener: &RefCell<FlowListener>,
) -> Rc<StatementV> {
    if *tokens.last().unwrap() == TokenV::Mark(14) {
        tokens.pop().unwrap();
        return parse_expression(tokens, 1, listener);
    } else {
        panic!(
            "expected -- while getting data from chain; found: {:?}",
            tokens.last().unwrap()
        )
    }
}
pub fn parse_program(tokens: &mut Vec<TokenV>, listener: &RefCell<FlowListener>) -> Rc<StatementV> {
    tokens.reverse();
    Rc::new(StatementV::Block(
        get_statements_of_block(parse_block(tokens, TokenV::EOF, listener)),
        BlockV::Evaluate,
    ))
}
fn get_statements_of_block(expr: Rc<StatementV>) -> Vec<Statement> {
    match expr.as_ref() {
        StatementV::Block(blocks, _) => blocks.clone(),
        v => vec![Statement::new(Rc::new(v.clone()))],
    }
}
pub fn parse_block(
    tokens: &mut Vec<TokenV>,
    closing_brace: TokenV,
    listener: &RefCell<FlowListener>,
) -> Rc<StatementV> {
    let mut clmn: TokenV;
    let mut stmts: Vec<Statement> = vec![];
    stmts.push(parse_statement(tokens, listener).into());
    if tokens.len() > 2 {
        clmn = tokens.pop().unwrap();
        if let TokenV::Dot(true) = clmn {
        } else if let TokenV::Dot(false) = clmn {
            tokens.pop().unwrap();
            if closing_brace == *tokens.last().unwrap() {
                tokens.pop().unwrap();
                return Rc::new(StatementV::Block(stmts, BlockV::Draft));
            }
        } else {
            panic!("expected comma or dot; found: {:?}", clmn)
        }
    } else {
        if let TokenV::Dot(false) = tokens.last().unwrap() {
            tokens.pop().unwrap();
            if closing_brace == *tokens.last().unwrap() {
                tokens.pop().unwrap();
            } else {
                panic!(
                    "expected closing brace: {:?}; found: {:?}",
                    closing_brace,
                    tokens.last().unwrap()
                );
            }
            return stmts[0].value.clone();
        } else {
            panic!("expected dot; found: {:?}", tokens.last().unwrap());
        }
    }
    while let TokenV::Dot(true) = clmn {
        if let TokenV::Dot(false) = tokens.last().unwrap() {
            tokens.pop().unwrap();
            if closing_brace == *tokens.last().unwrap() {
                tokens.pop().unwrap();
                break;
            } else {
                panic!(
                    "expected closing brace: {:?}; found: {:?}",
                    closing_brace,
                    tokens.last().unwrap()
                );
            }
        }
        let stmt = parse_statement(tokens, listener);
        stmts.push(stmt.into());
        if let TokenV::Dot(true) = tokens.last().unwrap() {
            clmn = tokens.pop().unwrap();
        }
    }
    Rc::new(StatementV::Block(stmts, BlockV::Draft))
}
fn parse_statement(tokens: &mut Vec<TokenV>, listener: &RefCell<FlowListener>) -> Rc<StatementV> {
    match tokens.pop().unwrap().clone() {
        TokenV::Mark(id) => match id {
            8 => parse_if_statement(tokens, listener),
            3 => {
                let name = tokens.pop().unwrap().get_string_from_name();
                let value: Statement = get_data_from_chain(tokens, listener).into();
                Rc::new(StatementV::Set { name, value })
            }
            16 => {
                let repeat = match tokens.pop().unwrap() {
                    TokenV::Mark(17) => true,
                    TokenV::Mark(18) => false,
                    _ => panic!("expected again or stop"),
                };
                Rc::new(StatementV::Jump(repeat))
            }
            19 => Rc::new(StatementV::In(RefCell::new(FlowStreamer::None))),
            20 => {
                let to_out: Rc<StatementV> = parse_statement(tokens, listener);
                Rc::new(StatementV::Out {
                    expr: to_out.into(),
                    to: listener.clone(),
                })
            }
            val => {
                tokens.push(TokenV::Mark(val));
                parse_expression(tokens, 1, listener)
            }
        },
        val => {
            tokens.push(val);
            parse_expression(tokens, 1, listener)
        }
    }
}
fn parse_if_statement(
    tokens: &mut Vec<TokenV>,
    listener: &RefCell<FlowListener>,
) -> Rc<StatementV> {
    let condition: Statement = parse_statement(tokens, listener).into();
    let if_: Statement = parse_statement(tokens, listener).into();
    if let TokenV::Mark(12) = tokens.last().unwrap() {
        tokens.pop();
        let else_: Statement = parse_statement(tokens, listener).into();
        Rc::new(StatementV::If(condition, if_, Some(else_)))
    } else {
        Rc::new(StatementV::If(condition, if_, None))
    }
}
fn parse_expression(
    tokens: &mut Vec<TokenV>,
    min_priority: u8,
    listener: &RefCell<FlowListener>,
) -> Rc<StatementV> {
    let mut left_expr: Statement = parse_primary(tokens, listener).into();
    loop {
        let op: TokenV = tokens.last().unwrap().clone();
        let priority = op.get_operation_priorety();
        if priority < min_priority || !op.is_operation() {
            break;
        }
        tokens.pop().unwrap();
        let right_expr: Statement = parse_expression(tokens, priority + 1, listener).into();
        left_expr = match &op {
            TokenV::Sign(_) => Rc::new(StatementV::OperationNumder(
                op.token_to_action_type(),
                left_expr,
                right_expr,
            ))
            .into(),
            TokenV::Comparsion(_) => Rc::new(StatementV::Comparsion(
                op.token_to_comparsion_type(),
                left_expr,
                right_expr,
            ))
            .into(),
            TokenV::Mark(1 | 7 | 9) => Rc::new(StatementV::OperationBool(
                op.token_to_action_type(),
                left_expr,
                Some(right_expr),
            ))
            .into(),
            _ => panic!("Invalid operation to operate"),
        };
    }
    left_expr.value
}
fn parse_primary(tokens: &mut Vec<TokenV>, listener: &RefCell<FlowListener>) -> Rc<StatementV> {
    if tokens.is_empty() {
        panic!("Invalid expression in primary");
    }
    let tk: TokenV = tokens.pop().unwrap();
    match tk {
        TokenV::Brackets { id, is_opened } => {
            if is_opened {
                match id {
                    1 => {
                        let expr: Rc<StatementV> = parse_statement(tokens, listener);
                        if let TokenV::Brackets {
                            id: 1,
                            is_opened: false,
                        } = tokens.pop().unwrap()
                        {
                        } else {
                            println!("{:?}", tokens);
                            panic!("Mismatched ) paren");
                        }
                        expr
                    }
                    3 => {
                        let expr: Rc<StatementV> = parse_block(
                            tokens,
                            TokenV::Brackets {
                                id: 3,
                                is_opened: false,
                            },
                            listener,
                        );
                        Rc::new(StatementV::Block(
                            get_statements_of_block(expr),
                            BlockV::Evaluate,
                        ))
                    }
                    4 => {
                        parse_block(tokens, TokenV::Brackets { id: 4, is_opened: false }, listener)
                    }    
                    _ => panic!("unexpected brace id"),
                }
            } else {
                panic!("closing paren found while no opening was: {:?}", tk)
            }
        }
        TokenV::Mark(1) | TokenV::Sign(2) => {
            let expr: Statement = parse_expression(tokens, 5, listener).into();
            Rc::new(StatementV::OperationBool(ActionV::Not, expr, None))
        }
        TokenV::Number(val) => Rc::new(StatementV::Number(val)),
        TokenV::Bool(val) => Rc::new(StatementV::Bool(val)),
        TokenV::Name(name) => Rc::new(StatementV::Name(name)),
        TokenV::EOF => {
            tokens.insert(0, TokenV::EOF);
            Rc::new(StatementV::Nil)
        }
        _ => panic!(
            "Unexpected token:  {:?}; tokens: {:?}",
            tk,
            tokens.iter().rev().collect::<Vec<&TokenV>>()
        ),
    }
}
