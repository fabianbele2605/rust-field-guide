#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,
    Print,
    Ident(String),
    Number(i64),
    Equals,
    Plus,
    Semicolon,
    LParen,
    RParen,
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = source.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        if c.is_alphabetic() {
            let start = i;
            while i < chars.len() && chars[i].is_alphanumeric() {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            match word.as_str() {
                "let" => tokens.push(Token::Let),
                "print" => tokens.push(Token::Print),
                _ => tokens.push(Token::Ident(word)),
            }
            continue;
        }

        if c.is_ascii_digit() {
            let start = i;
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }
            let number: String = chars[start..i].iter().collect();
            tokens.push(Token::Number(number.parse().unwrap()));
            continue;
        }

        match c {
            '=' => tokens.push(Token::Equals),
            '+' => tokens.push(Token::Plus),
            ';' => tokens.push(Token::Semicolon),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            _ => {}
        }
        i += 1;
    }

    tokens
}