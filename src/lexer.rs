use std::collections::HashMap;

use crate::types::Token;

pub fn tokenize_code(eq: String, keywords: &HashMap<String,u8>) -> Vec<Token> {
    let mut chars: Vec<char> = eq.chars().rev().collect();
    let mut tokens: Vec<Token> = Vec::new();
    while chars.len() > 0 {
        match chars.last().unwrap() {
            '\n' | '\r' | '\t' | ' ' => {chars.pop().unwrap();},

            val if val.is_ascii_punctuation() => tokens.append(&mut tokenize_symbol(&mut chars)),

            '0'..='9' => tokens.push(tokenize_number(&mut chars)),

            '_' | 'A'..='Z' | 'a'..='z' => tokens.push(tokenize_name(&mut chars,keywords)),
            _ => println!("symbol not recognized: {}", chars.last().unwrap()),
        }
    }
    tokens.push(Token::EOF);
    tokens
}
fn tokenize_name(chars: &mut Vec<char>,keywords: &HashMap<String,u8>) -> Token {
    let mut name: String = String::new();
    loop {
        if chars.len() > 0 {
            if !chars.last().unwrap().is_alphanumeric() || chars.last().unwrap() == &'_' {
                break;
            }
        } else {
            break;
        }
        let i = chars.pop().unwrap();
        name.push(i);
    }
    match keywords.get(name.as_str()) {
        Some(val) => Token::Mark(*val),
        None => Token::Name(name),
    }

}
fn tokenize_number(chars: &mut Vec<char>) -> Token {
    let mut number: String = String::new();
    if chars.len() == 0 {
        return Token::Number(0);
    }
    loop {
        if chars.len() > 0 {
            if !chars.last().unwrap().is_numeric() {
                break;
            }
        } else {
            break;
        }
        let i = chars.pop().unwrap();
        number.push(i);
    }
    Token::Number(number.parse().unwrap())
}
fn tokenize_symbol(chars: &mut Vec<char>,) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut symbol_sequence:String = String::new();
    while chars.len() > 0 {
        if !chars.last().unwrap().is_ascii_punctuation()
        {
            break;
        }
        let i = chars.pop().unwrap();
        if let ',' | '.' = i {
            tokens.push(Token::Dot(i == ','));
            if symbol_sequence.len() == 0 {
                return tokens;
            }
            break;
        }
        symbol_sequence.push(i);
    }
    tokens.insert(0,match symbol_sequence.as_str() {

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
            "unexpected symbol sequence: {}",
            symbol_sequence
        ),
    });
    tokens
}
