use std::collections::HashMap;

use crate::types::TokenV;

pub fn tokenize_code(eq: String, keywords: &HashMap<String,u8>) -> Vec<TokenV> {
    let mut chars: Vec<char> = eq.chars().rev().collect();
    let mut tokens: Vec<TokenV> = Vec::new();
    while chars.len() > 0 {
        match chars.last().unwrap() {
            '\n' | '\r' | '\t' | ' ' => {chars.pop().unwrap();},

            val if val.is_ascii_punctuation() => tokens.append(&mut tokenize_symbol(&mut chars)),

            '0'..='9' => tokens.push(tokenize_number(&mut chars)),

            '_' | 'A'..='Z' | 'a'..='z' => tokens.push(tokenize_name(&mut chars,keywords)),
            _ => println!("symbol not recognized: {}", chars.last().unwrap()),
        }
    }
    tokens.push(TokenV::EOF);
    tokens
}
fn tokenize_name(chars: &mut Vec<char>,keywords: &HashMap<String,u8>) -> TokenV {
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
        Some(val) => TokenV::Mark(*val),
        None => TokenV::Name(name),
    }

}
fn tokenize_number(chars: &mut Vec<char>) -> TokenV {
    let mut number: String = String::new();
    if chars.len() == 0 {
        return TokenV::Number(0);
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
    TokenV::Number(number.parse().unwrap())
}
fn tokenize_symbol(chars: &mut Vec<char>,) -> Vec<TokenV> {
    let mut tokens: Vec<TokenV> = Vec::new();
    let mut symbol_sequence:String = String::new();
    while chars.len() > 0 {
        if !chars.last().unwrap().is_ascii_punctuation()
        {
            break;
        }
        let i = chars.pop().unwrap();
        if let ',' | '.' = i {
            tokens.push(TokenV::Dot(i == ','));
            if symbol_sequence.len() == 0 {
                return tokens;
            }
            break;
        }
        symbol_sequence.push(i);
    }
    tokens.insert(0,match symbol_sequence.as_str() {

        "(" => TokenV::Brackets {id: 1,  is_opened: true},
        ")" => TokenV::Brackets {id: 1, is_opened: false},
        "[" => TokenV::Brackets {id: 2,  is_opened: true},
        "]" => TokenV::Brackets {id: 2, is_opened: false},
        "{" => TokenV::Brackets {id: 3,  is_opened: true},
        "}" => TokenV::Brackets {id: 3, is_opened: false},
        "<" => TokenV::Brackets { id: 4, is_opened: true },
        ">" => TokenV::Brackets { id: 4, is_opened: false },

        "!!" => TokenV::Bool(false),
        "==" => TokenV::Bool(true),

        "~" => TokenV::Mark(0),
        "!" => TokenV::Mark(1),
        "@" => TokenV::Mark(2),
        "#" => TokenV::Mark(3),
        "$" => TokenV::Mark(4),
        "%" => TokenV::Mark(5),
        "^" => TokenV::Mark(6),
        "&" => TokenV::Mark(7),
        "?" => TokenV::Mark(8),
        "|" => TokenV::Mark(9),

        "!:" => TokenV::Mark(11),
        "!-" => TokenV::Mark(12),
        "!+" => TokenV::Mark(13),

        "--" => TokenV::Mark(14),
        "=>" => TokenV::Mark(15),
        "->" => TokenV::Mark(16),

        "|+" => TokenV::Mark(17),
        "|-" => TokenV::Mark(18),
        "~+" => TokenV::Mark(19),
        "~-" => TokenV::Mark(20),

        "=" => TokenV::Comparsion(1),
        ">>" => TokenV::Comparsion(2),
        "<<" => TokenV::Comparsion(3),
        "!=" | "=!" => TokenV::Comparsion(4),
        ">=" => TokenV::Comparsion(5),
        "<=" => TokenV::Comparsion(6),

        "+" => TokenV::Sign(1),
        "-" => TokenV::Sign(2),
        "*" => TokenV::Sign(3),
        "/" => TokenV::Sign(4),

        _ => panic!(
            "unexpected symbol sequence: {}",
            symbol_sequence
        ),
    });
    tokens
}
