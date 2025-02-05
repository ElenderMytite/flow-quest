use crate::types::WordT;
use crate::types::TokenV;
use crate::types::Statement;
use crate::types::{ActionV, BlockV, StatementV};
use core::panic;
use std::rc::Rc;

pub fn shift<T>(vec: &mut Vec<T>) -> T {
    if vec.len() == 0 {
        panic!("trying to shift from vec with zero len")
    }
    vec.remove(0)
}
fn get_data_from_chain(tokens: &mut Vec<TokenV>) -> Rc<StatementV> 
{
    if tokens[0] == TokenV::Mark(14) 
    {
        shift(tokens);
        return parse_expression(tokens,1);
    }
    else {
        panic!("expected -- while getting data from chain; found: {:?}", tokens[0])
    }
}
pub fn parse_program(tokens: &mut Vec<TokenV>) -> Rc<StatementV> {
    Rc::new(StatementV::Block(get_statements_of_block(parse_block(tokens, TokenV::EOF)), BlockV::Evaluate))
}
fn get_statements_of_block(expr: Rc<StatementV>) -> Vec<Statement> {
    match expr.as_ref() {
        StatementV::Block(blocks, _) => blocks.clone(),
        v => vec![Statement::new(Rc::new(v.clone()))]
    }
}
pub fn parse_block(tokens: &mut Vec<TokenV>, closing_brace: TokenV) -> Rc<StatementV> {
    let mut clmn: TokenV;
    let mut stmts: Vec<Statement> = vec![];
    stmts.push(parse_statement(tokens).into());
    if tokens.len() > 2 {
        clmn = shift(tokens);
        if let TokenV::Dot(true) = clmn {
        } else if let TokenV::Dot(false) = clmn {
            shift(tokens);
            if closing_brace == tokens[0] {
                shift(tokens);
                return Rc::new(StatementV::Block(stmts, BlockV::Draft));
            }
        } else {
            panic!("expected comma or dot; found: {:?}", clmn)
        }
    } else {
        if let TokenV::Dot(false) = tokens[0] {
            shift(tokens);
            if closing_brace == tokens[0] {
                shift(tokens);
            } else {
                panic!(
                    "expected closing brace: {:?}; found: {:?}",
                    closing_brace, tokens[0]
                );
            }
            return stmts[0].value.clone();
        } else {
            panic!("expected dot; found: {:?}", tokens[0]);
        }
    }
    while let TokenV::Dot(true) = clmn {
        if let TokenV::Dot(false) = tokens[0] {
            println!("shifted dot");
            shift(tokens);
            if closing_brace == tokens[0] {
                shift(tokens);
                break;
            } else {
                panic!(
                    "expected closing brace: {:?}; found: {:?}",
                    closing_brace, tokens[0]
                );
            }
        }
        let stmt = parse_statement(tokens);
        stmts.push(stmt.into());
        if let TokenV::Dot(true) = tokens[0] {
            clmn = shift(tokens);
        }
    }
    Rc::new(StatementV::Block(stmts, BlockV::Draft))
}
fn parse_statement(tokens: &mut Vec<TokenV>) -> Rc<StatementV> {
    match tokens[0].clone() {
        TokenV::Mark(id) => match id {
            8 => {
                shift(tokens);
                parse_if_statement(tokens)
            }
            3 => {
                shift(tokens);
                let name = shift(tokens).get_string_from_name();
                let value: Statement = get_data_from_chain(tokens).into();
                Rc::new(StatementV::Set{name,value})
            }
            4 => {
                shift(tokens);
                let link = parse_expression(tokens, 1).into();
                Rc::new(StatementV::Define {
                    link,
                    like: 
                    match get_data_from_chain(tokens).as_ref() {
                        StatementV::Name(path) => {
                            path.clone()
                        }
                        _ => panic!("expected name in defining;"),
                    }
                })
            }
            5 => {
                shift(tokens);
                let name = shift(tokens).get_string_from_name();
                let value = get_data_from_chain(tokens);
                Rc::new(StatementV::Assign(name, value.into()))
            }
            _ => parse_expression(tokens, 1),
        },
        TokenV::Keyword(type_) => {
            shift(tokens);
            parse_keyword_statement(tokens, type_.clone())
        }
        _ => parse_expression(tokens, 1),
    }
}
fn parse_if_statement(tokens: &mut Vec<TokenV>) -> Rc<StatementV> {
    let condition: Statement = parse_statement(tokens).into();
    let if_: Statement = parse_statement(tokens).into();
    if let TokenV::Mark(12) = tokens[0] {
        tokens.remove(0);
        let else_: Statement = parse_statement(tokens).into();
        Rc::new(StatementV::If(condition, if_, Some(else_)))
    } else {
        Rc::new(StatementV::If(condition, if_, None))
    }
}
fn parse_keyword_statement(tokens: &mut Vec<TokenV>, keyword: WordT) -> Rc<StatementV> {
    match keyword {
        WordT::In => {
            if tokens.len() > 0 {
                if let TokenV::Mark(14) = tokens[0] {
                    shift(tokens);
                    if let TokenV::Name(..) = tokens[0] {
                        let tk = shift(tokens);
                        return Rc::new(StatementV::In(String::from(
                            tk.get_string_from_name(),
                        )));
                    }
                }
            }
            Rc::new(StatementV::In(String::from("-1")))
        }
        WordT::Out => {
            let to_out: Rc<StatementV> = parse_statement(tokens);
            if let TokenV::Mark(14) = tokens[0] {
                shift(tokens);
                let path = shift(tokens).get_string_from_name();
                return Rc::new(StatementV::OutExpr {
                    expr: to_out.into(),
                    like: Some(path),
                });
            };
            Rc::new(StatementV::OutExpr {
                expr: to_out.into(),
                like: None,
            })
        }
        WordT::Go => {
            let repeat: bool = match shift(tokens) {
                TokenV::Keyword(WordT::Again) => true,
                TokenV::Keyword(WordT::Stop) => false,
                _ => panic!("expected again or stop"),

            };
            Rc::new(StatementV::Jump(repeat))
        }
        _ => panic!(
            "again and stop can be used with do only; current keyword: {:?}",
            keyword
        ),
    }
}
fn parse_expression(tokens: &mut Vec<TokenV>, min_priority: u8) -> Rc<StatementV> {
    let mut left_expr: Statement = parse_primary(tokens).into();
    loop {
        let op: TokenV = tokens[0].clone();
        let priority = op.get_operation_priorety();
        if priority < min_priority || !op.is_operation() {
            break;
        }
        shift(tokens);
        let right_expr: Statement = parse_expression(tokens, priority + 1).into();
        left_expr = match &op {
            TokenV::Sign(_) => Rc::new(StatementV::OperationNumder(
                op.token_to_action_type(),
                left_expr,
                right_expr,
            )).into(),
            TokenV::Comparsion(_) => Rc::new(StatementV::Comparsion(
                op.token_to_comparsion_type(),
                left_expr,
                right_expr,
            )).into(),
            TokenV::Mark(1 | 7 | 9) => Rc::new(StatementV::OperationBool(
                op.token_to_action_type(),
                left_expr,
                Some(right_expr),
            )).into(),
            _ => panic!("Invalid operation to operate"),
        };
    }
    left_expr.value
}
fn parse_primary(tokens: &mut Vec<TokenV>) -> Rc<StatementV> {
    if tokens.is_empty() {
        panic!("Invalid expression in primary");
    }
    let tk: TokenV = shift(tokens);
    match tk {
        TokenV::Brackets { id, is_opened } => {
            if is_opened {
                match id {
                    1 => {
                        let expr: Rc<StatementV> = parse_statement(tokens);
                        if let TokenV::Brackets {
                            id: 1,
                            is_opened: false,
                        } = shift(tokens)
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
                        );
                        Rc::new(StatementV::Block(
                            get_statements_of_block(expr),
                            BlockV::Evaluate,
                        ))
                    }
                    _ => panic!("unexpected brace id"),
                }
            } else {
                panic!("closing paren found while no opening was: {:?}", tk)
            }
        }
        TokenV::Comparsion(3) => {
            let expr: Rc<StatementV> = parse_block(tokens, TokenV::Comparsion(2));
            expr
        }
        TokenV::Mark(1) => {
            let expr: Statement = parse_expression(tokens, 5).into();
            Rc::new(StatementV::OperationBool(ActionV::Not, expr, None))
        }
        TokenV::Number(val) => {
            Rc::new(StatementV::Number(val))
        }
        TokenV::Bool(val) => {
            Rc::new(StatementV::Bool(val))
        }
        TokenV::Name(name) => {
            Rc::new(StatementV::Name(name))
        }
        TokenV::EOF => {
            tokens.insert(0, TokenV::EOF);
            Rc::new(StatementV::Nil)
        }
        _ => panic!("Unexpected token:  {:?}; tokens: {:?}", tk, tokens),
    }
}
