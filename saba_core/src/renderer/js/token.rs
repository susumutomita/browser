use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    Punctuator(char),
    Number(u64),
}

pub struct JsLexer {
    pos: usize,
    input: Vec<char>,
}

impl JsLexer {
    pub fn new(js: String) -> Self {
        Self {
            pos: 0,
            input: js.chars().collect(),
        }
    }
}

impl Iterator for JsLexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while self.pos < self.input.len() {
            let c = self.input[self.pos];
            match c {
                '0'..='9' => {
                    let mut num = String::new();
                    while self.pos < self.input.len() && self.input[self.pos].is_digit(10) {
                        num.push(self.input[self.pos]);
                        self.pos += 1;
                    }
                    return Some(Token::Number(num.parse().unwrap()));
                }
                '+' | '-' | '*' | '/' | '%' | '(' | ')' | '{' | '}' | ';' | ',' => {
                    self.pos += 1;
                    return Some(Token::Punctuator(c));
                }
                _ => {
                    self.pos += 1;
                }
            }
        }

        None
    }
}
