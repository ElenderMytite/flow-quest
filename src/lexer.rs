use crate::types::{ActionType, ComparsionType};

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Token {
    From,
    Brackets { id: u8, is_opened: bool },
    Sign(u8),
    Number(isize),
    Name(String),
    Mark(u8),
    Keyword(KeyWord),
    Comparsion(u8),
    Range(i8),
    Colon(bool),
    Dot(bool),
    EOF,
}
impl Token {
    pub fn get_int_value(&self) -> isize {
        match &self {
            Token::Number(v) => *v,
            Token::Name(_) => panic!("trying to get name"),
            Token::Keyword(_) => panic!("trying to get keyword"),
            _ => panic!("Unexpected token while trying to get int value"),
        }
    }
    pub fn get_string_from_name(&self) -> String {
        match &self {
            Token::Name(name) => name.clone(),
            _ => panic!("expected name"),
        }
    }
    pub fn is_operation(&self) -> bool {
        //print!("{:?}", self);
        match &self {
            Token::Mark(1 | 7 | 9) | Token::Comparsion(_) | Token::Sign(_) => true,
            _ => false,
        }
    }
    pub fn get_operation_priorety(&self) -> u8 {
        match &self {
            Token::Comparsion(_) => 1,
            Token::Sign(1..=2) => 5,
            Token::Sign(3..=4) => 6,
            Token::Mark(7) => 2,
            Token::Mark(9) => 3,
            Token::Mark(1) => 4,
            _ => 0,
        }
    }
    pub fn token_to_action_type(&self) -> ActionType {
        match &self {
            Token::Sign(1) => ActionType::Plus,
            Token::Sign(2) => ActionType::Minus,
            Token::Sign(3) => ActionType::Multiply,
            Token::Sign(4) => ActionType::Divide,
            Token::Mark(1) => ActionType::Not,
            Token::Mark(7) => ActionType::And,
            Token::Mark(9) => ActionType::Or,
            _ => panic!("invalid action type"),
        }
    }
    pub fn token_to_comparsion_type(&self) -> ComparsionType {
        match &self {
            Token::Comparsion(id) => match id {
                1 => ComparsionType::Equal,
                2 => ComparsionType::Greater,
                3 => ComparsionType::Less,
                4 => ComparsionType::NotEqual,
                5 => ComparsionType::GreaterOrEqual,
                6 => ComparsionType::LessOrEqual,
                _ => panic!("invalid comparsion type id"),
            },
            _ => panic!("expected comparsion"),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum KeyWord {
    In,
    Out,
    Do,
    Stop,
    Again,
}
pub fn tokenize_code(eq: String) -> Vec<Token> {
    let mut chars: Vec<char> = eq.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();
    while chars.len() > 0 {
        let i = chars.remove(0);
        // println!("i: {i}");
        match i {
            '_' => tokens.push(Token::From),

            '(' => tokens.push(Token::Brackets {
                id: 1,
                is_opened: true,
            }),
            ')' => tokens.push(Token::Brackets {
                id: 1,
                is_opened: false,
            }),

            '[' => tokens.push(Token::Brackets {
                id: 2,
                is_opened: true,
            }),
            ']' => tokens.push(Token::Brackets {
                id: 2,
                is_opened: false,
            }),

            '{' => tokens.push(Token::Brackets {
                id: 3,
                is_opened: true,
            }),
            '}' => tokens.push(Token::Brackets {
                id: 3,
                is_opened: false,
            }),

            '+' => tokens.push(Token::Sign(1)),
            '-' => tokens.push(tokenize_multi_symbol(&mut chars, i)),
            '*' => tokens.push(Token::Sign(3)),
            '/' => tokens.push(Token::Sign(4)),

            '=' => tokens.push(tokenize_multi_symbol(&mut chars, i)),
            '<' => tokens.push(tokenize_multi_symbol(&mut chars, i)),
            '>' => tokens.push(tokenize_multi_symbol(&mut chars, i)),

            '!' => tokens.push(tokenize_multi_symbol(&mut chars, i)),
            '~' => tokens.push(Token::Mark(13)),
            '@' => tokens.push(Token::Mark(2)),
            '#' => tokens.push(Token::Mark(3)),
            '$' => tokens.push(Token::Mark(4)),
            '%' => tokens.push(Token::Mark(5)),
            '^' => tokens.push(Token::Mark(6)),
            '&' => tokens.push(Token::Mark(7)),
            '?' => tokens.push(Token::Mark(8)),
            '|' => tokens.push(Token::Mark(9)),

            '.' => tokens.push(tokenize_multi_symbol(&mut chars, i)),
            ',' => tokens.push(tokenize_multi_symbol(&mut chars, i)),

            '\n' | '\r' | ' ' => (),

            '0'..='9' => tokens.push(tokenize_number(&mut chars, i)),
            'A'..='Z' => tokens.push(tokenize_name(&mut chars, i)),
            'a'..='z' => tokens.push(tokenize_name(&mut chars, i)),

            _ => println!("non declared symbol"),
        }
    }
    tokens.push(Token::EOF);
    tokens
}
fn tokenize_name(chars: &mut Vec<char>, first: char) -> Token {
    let mut name: String = String::from(first);
    loop {
        if chars.len() > 0 {
            if !chars[0].is_alphanumeric() || chars[0] == '_' {
                break;
            }
        } else {
            break;
        }
        let i = chars.remove(0);
        name.push(i);
    }
    match name.to_lowercase().as_str() {
        "in" => Token::Keyword(KeyWord::In),
        "out" => Token::Keyword(KeyWord::Out),
        "do" => Token::Keyword(KeyWord::Do),
        "stop" => Token::Keyword(KeyWord::Stop),
        "again" => Token::Keyword(KeyWord::Again),
        _ => Token::Name(name),
    }
}
#[allow(dead_code)]
fn tokenize_number(chars: &mut Vec<char>, first: char) -> Token {
    let mut number: String = String::from(first);
    if chars.len() == 0 {
        return Token::Number(0);
    }
    loop {
        if chars.len() > 0 {
            if !chars[0].is_numeric() {
                break;
            }
        } else {
            break;
        }
        let i = chars.remove(0);
        number.push(i);
    }
    Token::Number(number.parse().unwrap())
}
#[allow(dead_code)]
fn tokenize_multi_symbol(chars: &mut Vec<char>, first: char) -> Token {
    if chars.len() > 2 {
        match first {
            '-' => match chars[0] {
                '-' => {
                    chars.remove(0);
                    Token::Mark(14)
                }
                _ => Token::Sign(2),
            },
            '!' => match chars[0] {
                '!' => {
                    chars.remove(0);
                    Token::Mark(10)
                }
                '=' => {
                    chars.remove(0);
                    Token::Comparsion(4)
                }
                '-' => {
                    chars.remove(0);
                    Token::Mark(12)
                }
                _ => Token::Mark(1),
            },
            '=' => match chars[0] {
                '!' => {
                    chars.remove(0);
                    Token::Comparsion(4)
                }
                '=' => {
                    chars.remove(0);
                    Token::Mark(11)
                }
                '>' => {
                    chars.remove(0);
                    Token::Comparsion(5)
                }
                '<' => {
                    chars.remove(0);
                    Token::Comparsion(6)
                }
                _ => Token::Comparsion(1),
            },
            '>' => match chars[0] {
                '=' => {
                    chars.remove(0);
                    Token::Comparsion(5)
                }
                _ => Token::Comparsion(2),
            },
            '<' => match chars[0] {
                '=' => {
                    chars.remove(0);
                    Token::Comparsion(6)
                }
                _ => Token::Comparsion(3),
            },
            '.' => match chars[0] {
                '.' => Token::Range(0),
                ',' => Token::Range(1),
                _ => Token::Dot(false),
            },
            ',' => match chars[0] {
                '.' => Token::Range(-1),
                ',' => Token::Range(2),
                _ => Token::Dot(true),
            },
            _ => panic!("cant two symbol this: {}", first),
        }
    } else {
        match first {
            ',' => Token::Dot(true),
            '.' => Token::Dot(false),
            '<' => Token::Comparsion(3),
            '>' => Token::Comparsion(2),
            '=' => Token::Comparsion(1),
            '!' => Token::Mark(1),
            '-' => Token::Sign(2),
            _ => panic!("cant two symbol this: {}", first),
        }
    }
}
