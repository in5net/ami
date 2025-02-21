use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(String),
    Newline,
    EOF,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Number(value) => write!(f, "{}", value),
            Token::Newline => write!(f, "'\\n'"),
            Token::EOF => write!(f, "<eof>"),
        }
    }
}
