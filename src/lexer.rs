use crate::Token;
use Token::*;

pub struct Lexer {
    source: String,
    index: usize,
    current_char: char,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            index: 0,
            current_char: source.chars().nth(0).unwrap_or('\0'),
            source,
        }
    }

    fn advance(&mut self) {
        self.index += 1;
        let ch = self.source.chars().nth(self.index).unwrap_or('\0');
        self.current_char = ch;
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut token = self.next_token();
        while token != EOF {
            tokens.push(token);
            token = self.next_token();
        }
        tokens.push(token);
        tokens
    }

    pub fn next_token(&mut self) -> Token {
        while matches!(self.current_char, ' ' | '\t' | '\r') {
            self.advance();
        }

        match self.current_char {
            '0'..='9' => self.number(),
            '+' => {
                self.advance();
                Plus
            }
            '-' => {
                self.advance();
                Minus
            }
            '\n' => {
                self.advance();
                Newline
            }
            '\0' => EOF,
            _ => panic!("'{}' is not a valid character", self.current_char),
        }
    }

    fn number(&mut self) -> Token {
        let mut num_str: String = self.current_char.to_string();
        self.advance();

        while "0123456789.".contains(self.current_char) {
            num_str.push(self.current_char);
            self.advance();
        }

        Token::Number(num_str)
    }
}
