use crate::{AmiError, Token, TokenType};

use TokenType::*;

pub struct Lexer {
    source: String,
    index: usize,
    current_char: char,
}

const SUPERSCRIPT: &str = "ᵃᵇᶜᵈᵉᶠᵍʰⁱʲᵏˡᵐⁿᵒᵖʳˢᵗᵘᵛʷˣʸᶻᴬᴮᶜᴰᴱᶠᴳᴴᴵᴶᴷᴸᴹᴺᴼᴾᴿˢᵀᵁⱽᵂˣʸᶻ⁰¹²³⁴⁵⁶⁷⁸⁹⁺⁻⁼⁽⁾";
const NORMALSCRIPT: &str = "abcdefghijklmnoprstuvwxyzABCDEFGHIJKLMNOPRSTUVWXYZ0123456789+-=()";

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
            'a'..='z' | 'A'..='Z' | '_' | 'Α'..='ω' | '∞' => self.word(),
            ch if SUPERSCRIPT.contains(ch) => self.superscript(),
            '=' => {
                self.advance();
                Ok(Token {
                    ty: Eq,
                    range: start..self.index,
                })
            }
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
                Ok(Token {
                    ty: Slash,
                    range: start..self.index,
                })
            }
            '÷' => {
                self.advance();
                Ok(Token {
                    ty: Divide,
                    range: start..self.index,
                })
            }
            '%' => {
                self.advance();
                Ok(Token {
                    ty: Percent,
                    range: start..self.index,
                })
            }
            '^' => {
                self.advance();
                Ok(Token {
                    ty: Carrot,
                    range: start..self.index,
                })
            }
            '√' => {
                self.advance();
                Ok(Token {
                    ty: Sqrt,
                    range: start..self.index,
                })
            }
            '∛' => {
                self.advance();
                Ok(Token {
                    ty: Cbrt,
                    range: start..self.index,
                })
            }
            '∜' => {
                self.advance();
                Ok(Token {
                    ty: Fort,
                    range: start..self.index,
                })
            }
            '°' => {
                self.advance();
                Ok(Token {
                    ty: Degree,
                    range: start..self.index,
                })
            }
            '!' => {
                self.advance();
                Ok(Token {
                    ty: Exclamation,
                    range: start..self.index,
                })
            }
            '(' => {
                self.advance();
                Ok(Token {
                    ty: LeftParen,
                    range: start..self.index,
                })
            }
            ')' => {
                self.advance();
                Ok(Token {
                    ty: RightParen,
                    range: start..self.index,
                })
            }
            '{' => {
                self.advance();
                Ok(Token {
                    ty: LeftBrace,
                    range: start..self.index,
                })
            }
            '}' => {
                self.advance();
                Ok(Token {
                    ty: RightBrace,
                    range: start..self.index,
                })
            }
            '|' => {
                self.advance();
                Ok(Token {
                    ty: Pipe,
                    range: start..self.index,
                })
            }
            '⌊' => {
                self.advance();
                Ok(Token {
                    ty: LeftFloor,
                    range: start..self.index,
                })
            }
            '⌋' => {
                self.advance();
                Ok(Token {
                    ty: RightFloor,
                    range: start..self.index,
                })
            }
            '⌈' => {
                self.advance();
                Ok(Token {
                    ty: LeftCeil,
                    range: start..self.index,
                })
            }
            '⌉' => {
                self.advance();
                Ok(Token {
                    ty: RightCeil,
                    range: start..self.index,
                })
            }
            ',' => {
                self.advance();
                Ok(Token {
                    ty: Comma,
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

    fn word(&mut self) -> LexResult {
        let start = self.index;
        let mut word = self.current_char.to_string();
        self.advance();

        while self.current_char != '\0' {
            match self.current_char {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | 'Α'..='ω' | '∞' => {
                    word.push(self.current_char);
                    self.advance();
                }
                _ => break,
            };
        }

        Ok(Token {
            ty: match word.as_str() {
                "mod" => Mod,
                _ => Identifier(word.into()),
            },
            range: start..self.index,
        })
    }

    fn superscript(&mut self) -> LexResult {
        let start = self.index;
        let mut source = String::new();

        while self.current_char != '\0' {
            match SUPERSCRIPT.chars().position(|ch| ch == self.current_char) {
                Some(index) => {
                    let normal_char = NORMALSCRIPT.chars().nth(index).unwrap();
                    source.push(normal_char);
                    self.advance();
                }
                None => break,
            };
        }

        let mut lexer = Lexer::new(source);
        let mut tokens = lexer.lex()?;
        tokens.pop();
        Ok(Token {
            ty: Superscript(tokens),
            range: start..self.index,
        })
    }
}
