use crate::types::{ActionV, ComparsionV};

use super::Token;

impl Token {
    pub fn is_operation(&self) -> bool {
        //print!("{:?}", self);
        match &self {
            Token::Mark(1 | 7 | 9) | Token::Comparsion(_) | Token::Sign(_) => true,
            _ => false,
        }
    }
    pub fn name_id(&self) -> usize {
        match &self {
            Token::Name(id) => *id,
            _ => panic!("expected name token"),
        }
    }
    pub fn get_operation_priorety(&self) -> u8 {
        match &self {
            Token::Comparsion(_) => 4,
            Token::Sign(1..=2) => 5,
            Token::Sign(3..=5) => 6,
            Token::Mark(7) => 2,
            Token::Mark(9) => 1,
            Token::Mark(1) => 3,
            _ => 0,
        }
    }
    pub fn token_to_action_type(&self) -> ActionV {
        match &self {
            Token::Sign(1) => ActionV::Add,
            Token::Sign(2) => ActionV::Sub,
            Token::Sign(3) => ActionV::Mul,
            Token::Sign(4) => ActionV::Div,
            Token::Sign(5) => ActionV::Mod,
            Token::Mark(1) => ActionV::Not,
            Token::Mark(7) => ActionV::And,
            Token::Mark(9) => ActionV::Or,
            _ => panic!("invalid action type"),
        }
    }
    pub fn token_to_comparsion_type(&self) -> ComparsionV {
        match &self {
            Token::Comparsion(id) => match id {
                1 => ComparsionV::Equal,
                2 => ComparsionV::Greater,
                3 => ComparsionV::Less,
                4 => ComparsionV::NotEqual,
                5 => ComparsionV::GreaterOrEqual,
                6 => ComparsionV::LessOrEqual,
                _ => panic!("invalid comparsion type id"),
            },
            _ => panic!("expected comparsion"),
        }
    }
}