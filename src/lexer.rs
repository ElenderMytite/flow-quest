use std::collections::HashMap;

use crate::types::Token;

pub fn tokenize_code(eq: String, keywords: &HashMap<String,u8>) -> Vec<Token> {
    let chars: Vec<char> = eq.chars().collect();
    let mut index: usize = 0;
    let mut tokens: Vec<Token> = Vec::new();
    let mut names: HashMap<String, usize> = HashMap::new();
    while chars.len() > index {
        match chars[index] {
            '\n' | '\r' | '\t' | ' ' => {index += 1;},

            val if val.is_ascii_punctuation() => tokens.append(&mut tokenize_symbol(&chars, &mut index)),

            '0'..='9' => tokens.push(tokenize_number(& chars,&mut index)),

            '_' | 'A'..='Z' | 'a'..='z' => tokens.push(tokenize_name(&chars,&mut index,&mut names, keywords)),
            _ => println!("symbol not recognized: {}", chars.last().unwrap()),
        }
    }
    tokens.push(Token::EOF);
    tokens
}
fn tokenize_name(chars: &Vec<char>,index: &mut usize, names: &mut HashMap<String,usize>, keywords: &HashMap<String,u8>) -> Token {
    let mut name: Box<str> = Box::from("");
    loop {
        if chars.len() > *index {
            if !chars[*index].is_alphanumeric() || chars.last().unwrap() == &'_' {
                break;
            }
        } else {
            break;
        }
        let i = chars[*index];
        *index += 1;
        name = format!("{}{}",name, &i.to_string()).into_boxed_str();
    }
    match keywords.get(&name.to_string()) {
        Some(val) => Token::Mark(*val),
        None => {match names.get(&name.to_string()) {
            Some(id) => Token::Name(id.clone()),
            None => {
                names.insert(name.to_string(), names.len());
                Token::Name(names.len() - 1)
            }}
        }
    }
}
fn tokenize_number(chars: &Vec<char>, index: &mut usize) -> Token {
    let mut number: String = String::new();
    if chars.len() == *index {
        return Token::Number(0);
    }
    loop {
        if chars.len() > *index {
            if !chars[*index].is_numeric() {
                break;
            }
        } else {
            break;
        }
        let i = chars[*index];
        *index += 1;
        number.push(i);
    }
    Token::Number(number.parse().unwrap())
}
fn tokenize_symbol(chars: &Vec<char>,index: &mut usize) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut symbol_sequence:String = String::new();
    while chars.len() > *index {
        if !chars[*index].is_ascii_punctuation()
        {
            break;
        }
        let i = chars[*index];
        *index += 1;
        if let ',' | '.' = i {
            tokens.push(Token::Dot(i == ','));
            if symbol_sequence.len() == 0 {
                return tokens;
            }
            break;
        }
        symbol_sequence.push(i);
    }
    tokens.push(match symbol_sequence.as_str() {

        "(" => Token::Brackets {id: 1,  is_opened: true},
        ")" => Token::Brackets {id: 1, is_opened: false},
        "[" => Token::Brackets {id: 2,  is_opened: true},
        "]" => Token::Brackets {id: 2, is_opened: false},
        "{" => Token::Brackets {id: 3,  is_opened: true},
        "}" => Token::Brackets {id: 3, is_opened: false},
        "<" => Token::Brackets { id: 4, is_opened: true },
        ">" => Token::Brackets { id: 4, is_opened: false },
        
        "!!" => Token::Bool(false),
        "==" => Token::Bool(true),
        
        "~" => Token::Mark(0),
        "!" => Token::Mark(1),
        "@" => Token::Mark(2),
        "#" => Token::Mark(3),
        "$" => Token::Mark(4),
        "^" => Token::Mark(6),
        "&" => Token::Mark(7),
        "?" => Token::Mark(8),
        "|" => Token::Mark(9),
        
        "!:" => Token::Mark(11),
        "!-" => Token::Mark(12),
        "!+" => Token::Mark(13),
        
        "--" => Token::Mark(14),
        "=>" => Token::Mark(15),
        "->" => Token::Mark(16),
        
        "|+" => Token::Mark(17),
        "|-" => Token::Mark(18),
        "~+" => Token::Mark(19),
        "~-" => Token::Mark(20),
        "@+" => Token::Mark(21),
        "@-" => Token::Mark(22),
        
        "=" => Token::Comparsion(1),
        ">>" => Token::Comparsion(2),
        "<<" => Token::Comparsion(3),
        "!=" | "=!" => Token::Comparsion(4),
        ">=" => Token::Comparsion(5),
        "<=" => Token::Comparsion(6),
        
        "+" => Token::Sign(1),
        "-" => Token::Sign(2),
        "*" => Token::Sign(3),
        "/" => Token::Sign(4),
        "%" => Token::Sign(5),
        
        _ => panic!(
            "unexpected symbol sequence: \"{}\"",
            symbol_sequence
        ),
    });
    tokens.iter().rev().cloned().collect()
}
