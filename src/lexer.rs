use crate::{AmiError, Token, TokenType};

use TokenType::*;

pub struct Lexer {
    source: String,
    index: usize,
    current_char: char,
}

type LexResult = Result<Token, AmiError>;

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

    fn error(&self, msg: String, reason: String, start: usize) -> LexResult {
        Err(AmiError {
            msg,
            reason,
            range: start..self.index,
        })
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, AmiError> {
        let mut tokens: Vec<Token> = vec![];
        let mut token = self.next_token()?;
        while token.ty != EOF {
            tokens.push(token);
            token = self.next_token()?;
        }
        tokens.push(token);
        Ok(tokens)
    }

    pub fn next_token(&mut self) -> LexResult {
        while matches!(self.current_char, ' ' | '\t' | '\r') {
            self.advance();
        }

        let start = self.index;
        match self.current_char {
            '0'..='9' => self.number(),
            '+' => {
                self.advance();
                Ok(Token {
                    ty: Plus,
                    range: start..self.index,
                })
            }
            '-' => {
                self.advance();
                Ok(Token {
                    ty: Minus,
                    range: start..self.index,
                })
            }
            '*' => {
                self.advance();
                Ok(Token {
                    ty: Star,
                    range: start..self.index,
                })
            }
            '∙' => {
                self.advance();
                Ok(Token {
                    ty: Dot,
                    range: start..self.index,
                })
            }
            '×' => {
                self.advance();
                Ok(Token {
                    ty: Cross,
                    range: start..self.index,
                })
            }
            '/' => {
                self.advance();
                if self.current_char == '/' {
                    while self.current_char != '\n' {
                        self.advance();
                    }
                    self.next_token()
                } else {
                    Ok(Token {
                        ty: Slash,
                        range: start..self.index,
                    })
                }
            }
            '÷' => {
                self.advance();
                Ok(Token {
                    ty: Divide,
                    range: start..self.index,
                })
            }
            '\n' => {
                self.advance();
                Ok(Token {
                    ty: Newline,
                    range: start..self.index,
                })
            }
            '\0' => Ok(Token {
                ty: EOF,
                range: start..self.index,
            }),
            _ => self.error(
                "invalid character".to_string(),
                format!("'{}' is not a valid character", self.current_char),
                start,
            ),
        }
    }

    fn number(&mut self) -> LexResult {
        let start = self.index;
        let mut num_str = self.current_char.to_string();
        self.advance();

        while "0123456789.".contains(self.current_char) {
            num_str.push(self.current_char);
            self.advance();
        }

        Ok(Token {
            ty: Number(num_str.into()),
            range: start..self.index,
        })
    }
}
