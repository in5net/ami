use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(String),
    Plus,
    Minus,
    Newline,
    EOF,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(value) => write!(f, "{}", value),
            Self::Plus => write!(f, "'+'"),
            Self::Minus => write!(f, "'-'"),
            Self::Newline => write!(f, "'\\n'"),
            Self::EOF => write!(f, "<eof>"),
        }
    }
}
