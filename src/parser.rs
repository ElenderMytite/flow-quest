use crate::lexer::KeyWord;
use crate::lexer::Token;
use crate::types::create_module_path;
use crate::types::ModulePath;
use crate::types::Statement;
use crate::types::{ActionType, BlockType, ExpressionType};
use core::panic;
use std::rc::Rc;
fn shift(vec: &mut Vec<Token>) -> Token {
    if vec.len() == 0 {
        panic!("trying to shift from vec with zero len")
    }
    vec.remove(0)
}
fn get_data_from_chain(tokens: &mut Vec<Token>) -> Rc<ExpressionType> 
{
    if tokens[0] == Token::Mark(14) 
    {
        shift(tokens);
        return parse_expression(tokens,1);
    }
    else {
        panic!("expected -- while getting data from chain; found: {:?}", tokens[0])
    }
}
fn get_module_name(tokens: &mut Vec<Token>, first: String) -> Box<ModulePath> {
    let mut is_running = true;
    if first.as_str() != "" {
        if let Token::From = tokens[0] {
            shift(tokens);
        } else {
            is_running = false
        }
    }
    let mut names: Vec<String> = vec![first];
    if names[0] == "" {
        names.pop();
    }
    while (tokens.len() > 0) & is_running {
        match shift(tokens) {
            Token::Name(name) => {
                names.push(name);
                if let Token::From = tokens[0] {
                    shift(tokens);
                } else {
                    is_running = false
                }
            }
            Token::Sign(id) => match id {
                3 => {
                    names.push(String::from("-all"));
                    if let Token::From = tokens[0] {
                        shift(tokens);
                    } else {
                        is_running = false
                    }
                }
                _ => panic!("unexpected token: {:?}; expected name or *", tokens[0]),
            },
            v => panic!("unexpected token: {:?}; expected name or *", v),
        }
    }
    {
        let omp = create_module_path(names);
        match omp {
            None => panic!("found none modulepath"),
            Some(mp) => mp,
        }
    }
}
pub fn parse_program(tokens: &mut Vec<Token>) -> Rc<ExpressionType> {
    Rc::new(ExpressionType::Block(get_statements_of_block(parse_block(tokens, Token::EOF)), BlockType::Evaluate))
}
fn get_statements_of_block(expr: Rc<ExpressionType>) -> Vec<Statement> {
    match expr.as_ref() {
        ExpressionType::Block(blocks, _) => blocks.clone(),
        v => vec![Statement::new(Rc::new(v.clone()))]
    }
}
pub fn parse_block(tokens: &mut Vec<Token>, closing_brace: Token) -> Rc<ExpressionType> {
    let mut clmn: Token;
    let mut stmts: Vec<Statement> = vec![];
    stmts.push(parse_statement(tokens).into());
    if tokens.len() > 2 {
        clmn = shift(tokens);
        if let Token::Dot(true) = clmn {
        } else if let Token::Dot(false) = clmn {
            shift(tokens);
            if closing_brace == tokens[0] {
                shift(tokens);
                return Rc::new(ExpressionType::Block(stmts, BlockType::Draft));
            }
        } else {
            panic!("expected comma or dot; found: {:?}", clmn)
        }
    } else {
        if let Token::Dot(false) = tokens[0] {
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
    while let Token::Dot(true) = clmn {
        if let Token::Dot(false) = tokens[0] {
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
        if let Token::Dot(true) = tokens[0] {
            clmn = shift(tokens);
        }
    }
    Rc::new(ExpressionType::Block(stmts, BlockType::Draft))
}
fn parse_statement(tokens: &mut Vec<Token>) -> Rc<ExpressionType> {
    match tokens[0].clone() {
        Token::Mark(id) => match id {
            8 => {
                shift(tokens);
                parse_if_statement(tokens)
            }
            4 => {
                shift(tokens);
                let value = parse_expression(tokens, 1).into();
                Rc::new(ExpressionType::Define {
                    value,
                    like: 
                    match get_data_from_chain(tokens).as_ref() {
                        ExpressionType::Name(path) => {
                            path.clone()
                        }
                        _ => panic!("expected name in defining; ", ),
                    }
                })
            }
            5 => {
                shift(tokens);
                let name = get_module_name(tokens, String::from(""));
                let value = get_data_from_chain(tokens);
                Rc::new(ExpressionType::Assign(name, value.into()))
            }
            _ => parse_expression(tokens, 1),
        },
        Token::Keyword(type_) => {
            shift(tokens);
            parse_keyword_statement(tokens, type_.clone())
        }
        _ => parse_expression(tokens, 1),
    }
}
fn parse_if_statement(tokens: &mut Vec<Token>) -> Rc<ExpressionType> {
    let condition: Statement = parse_statement(tokens).into();
    let if_: Statement = parse_statement(tokens).into();
    if let Token::Mark(12) = tokens[0] {
        tokens.remove(0);
        let else_: Statement = parse_statement(tokens).into();
        Rc::new(ExpressionType::If(condition, if_, Some(else_)))
    } else {
        Rc::new(ExpressionType::If(condition, if_, None))
    }
}
fn parse_keyword_statement(tokens: &mut Vec<Token>, keyword: KeyWord) -> Rc<ExpressionType> {
    match keyword {
        KeyWord::In => {
            if tokens.len() > 0 {
                if let Token::Mark(14) = tokens[0] {
                    shift(tokens);
                    if let Token::Name(..) = tokens[0] {
                        let tk = shift(tokens);
                        return Rc::new(ExpressionType::In(String::from(
                            tk.get_string_from_name(),
                        )));
                    }
                }
            }
            Rc::new(ExpressionType::In(String::from("-1")))
        }
        KeyWord::Out => {
            let to_out: Rc<ExpressionType> = parse_statement(tokens);
            if let Token::Mark(14) = tokens[0] {
                shift(tokens);
                let path = get_module_name(tokens, String::from(""));
                return Rc::new(ExpressionType::OutExpr {
                    expr: to_out.into(),
                    like: Some(path),
                });
            };
            Rc::new(ExpressionType::OutExpr {
                expr: to_out.into(),
                like: None,
            })
        }
        KeyWord::Do => {
            match tokens[0].clone() {
                Token::Keyword(word) => match word {
                    KeyWord::Again => {
                        shift(tokens);
                        Rc::new(ExpressionType::Jump(true))
                    }
                    KeyWord::Stop => {
                        shift(tokens);
                        Rc::new(ExpressionType::Jump(false))
                    }
                    _ => panic!("not implemented with this keyword: {:?}", word),
                },
                Token::Name(_) => {
                    panic!("not implemented yet DO with name")
                }
                _ => {
                    panic!("expected name or keyword; found: {:?}", tokens[0])
                }
            }
        }
        _ => panic!(
            "again and stop can be used with do only; current keyword: {:?}",
            keyword
        ),
    }
}
fn parse_expression(tokens: &mut Vec<Token>, min_priority: u8) -> Rc<ExpressionType> {
    let mut left_expr: Statement = parse_primary(tokens).into();
    loop {
        let op: Token = tokens[0].clone();
        let priority = op.get_operation_priorety();
        if priority < min_priority || !op.is_operation() {
            break;
        }
        shift(tokens);
        let right_expr: Statement = parse_expression(tokens, priority + 1).into();
        left_expr = match &op {
            Token::Sign(_) => Rc::new(ExpressionType::OperationNumder(
                op.token_to_action_type(),
                left_expr,
                right_expr,
            )).into(),
            Token::Comparsion(_) => Rc::new(ExpressionType::Comparsion(
                op.token_to_comparsion_type(),
                left_expr,
                right_expr,
            )).into(),
            Token::Mark(1 | 7 | 9) => Rc::new(ExpressionType::OperationBool(
                op.token_to_action_type(),
                left_expr,
                Some(right_expr),
            )).into(),
            _ => panic!("Invalid operation to operate"),
        };
    }
    left_expr.value
}
fn parse_primary(tokens: &mut Vec<Token>) -> Rc<ExpressionType> {
    if tokens.is_empty() {
        panic!("Invalid expression in primary");
    }
    let tk: Token = shift(tokens);
    match tk {
        Token::Brackets { id, is_opened } => {
            if is_opened {
                match id {
                    1 => {
                        let expr: Rc<ExpressionType> = parse_statement(tokens);
                        if let Token::Brackets {
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
                        let expr: Rc<ExpressionType> = parse_block(
                            tokens,
                            Token::Brackets {
                                id: 3,
                                is_opened: false,
                            },
                        );
                        Rc::new(ExpressionType::Block(
                            get_statements_of_block(expr),
                            BlockType::Evaluate,
                        ))
                    }
                    _ => panic!("unexpected brace id"),
                }
            } else {
                panic!("closing paren found while no opening was: {:?}", tk)
            }
        }
        Token::Comparsion(3) => {
            let expr: Rc<ExpressionType> = parse_block(tokens, Token::Comparsion(2));
            expr
        }
        Token::Mark(1) => {
            let expr: Statement = parse_expression(tokens, 5).into();
            Rc::new(ExpressionType::OperationBool(ActionType::Not, expr, None))
        }
        Token::Number(..) => {
            let num = tk.get_int_value();
            Rc::new(ExpressionType::Number(num))
        }
        Token::Name(first) => {
            let name = get_module_name(tokens, first);
            Rc::new(ExpressionType::Name(name))
        }
        Token::Mark(id) => match id {
            10 => Rc::new(ExpressionType::Bool(false)),
            11 => Rc::new(ExpressionType::Bool(true)),
            _ => panic!("unexpected mark while parsing primary: {:?}", tk),
        },
        Token::EOF => {
            tokens.insert(0, Token::EOF);
            Rc::new(ExpressionType::Nil)
        }
        _ => panic!("Unexpected token:  {:?}; tokens: {:?}", tk, tokens),
    }
}
