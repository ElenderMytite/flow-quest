use crate::{parser::shift, types::{TokenV, WordT}};

pub fn tokenize_code(eq: String) -> Vec<TokenV> {
    let mut chars: Vec<char> = eq.chars().collect();
    let mut tokens: Vec<TokenV> = Vec::new();
    while chars.len() > 0 {
        let i = shift(&mut chars);
        match i {
            '_' => tokens.push(TokenV::Skip),

            '(' => tokens.push(TokenV::Brackets {
                id: 1,
                is_opened: true,
            }),
            ')' => tokens.push(TokenV::Brackets {
                id: 1,
                is_opened: false,
            }),

            '[' => tokens.push(TokenV::Brackets {
                id: 2,
                is_opened: true,
            }),
            ']' => tokens.push(TokenV::Brackets {
                id: 2,
                is_opened: false,
            }),

            '{' => tokens.push(TokenV::Brackets {
                id: 3,
                is_opened: true,
            }),
            '}' => tokens.push(TokenV::Brackets {
                id: 3,
                is_opened: false,
            }),

            val if val.is_ascii_punctuation() => tokens.push(tokenize_symbol_combination(&mut chars, i)),

            '\n' | '\r' | ' ' => (),

            '0'..='9' => tokens.push(tokenize_number(&mut chars, i)),
            'A'..='Z' => tokens.push(tokenize_name(&mut chars, i)),
            'a'..='z' => tokens.push(tokenize_name(&mut chars, i)),

            _ => println!("non declared symbol"),
        }
    }
    tokens.push(TokenV::EOF);
    tokens
}
fn tokenize_name(chars: &mut Vec<char>, first: char) -> TokenV {
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
        "in" => TokenV::Keyword(WordT::In),
        "out" => TokenV::Keyword(WordT::Out),
        "do" => TokenV::Keyword(WordT::Go),
        "stop" => TokenV::Keyword(WordT::Stop),
        "again" => TokenV::Keyword(WordT::Again),
        _ => TokenV::Name(name),
    }
}
fn tokenize_number(chars: &mut Vec<char>, first: char) -> TokenV {
    let mut number: String = String::from(first);
    if chars.len() == 0 {
        return TokenV::Number(0);
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
    TokenV::Number(number.parse().unwrap())
}
#[allow(dead_code)]
fn tokenize_symbol_combination(chars: &mut Vec<char>, first: char) -> TokenV {
    let mut symbol_art:Vec<char> = vec![first];
    while chars.len() > 0 {
        if !chars[0].is_ascii_punctuation() {
            break;
        }
        let i = shift(chars);
        symbol_art.push(i);
        
    }
        match symbol_art.iter().collect::<String>().as_str() {
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


            
            
            "=" => TokenV::Comparsion(1),
            ">" => TokenV::Comparsion(2),
            "<" => TokenV::Comparsion(3),
            "!=" | "=!" => TokenV::Comparsion(4),
            ">=" => TokenV::Comparsion(5),
            "<=" => TokenV::Comparsion(6),

            "." => TokenV::Dot(false),
            "," => TokenV::Dot(true),

            "+" => TokenV::Sign(1),
            "-" => TokenV::Sign(2),
            "*" => TokenV::Sign(3),
            "/" => TokenV::Sign(4),

            _ => panic!("cant  symbol this: {}", symbol_art.iter().collect::<String>()),
    }
}
