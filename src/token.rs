use std::{fmt, ops::Range, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Number(Rc<str>),
    Plus,
    Minus,
    Newline,
    EOF,
}

impl fmt::Display for TokenType {
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

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub ty: TokenType,
    pub range: Range<usize>,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ty)
    }
}
