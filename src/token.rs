use std::{fmt, ops::Range, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Number(Rc<str>),
    Plus,
    Minus,
    Star,
    Dot,
    Cross,
    Slash,
    Divide,
    Percent,
    Carrot,
    Sqrt,
    Cbrt,
    Fort,
    Degree,
    Exclamation,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Pipe,
    LeftFloor,
    RightFloor,
    LeftCeil,
    RightCeil,
    Newline,
    EOF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(value) => write!(f, "{}", value),
            Self::Plus => write!(f, "'+'"),
            Self::Minus => write!(f, "'-'"),
            Self::Star => write!(f, "'*'"),
            Self::Dot => write!(f, "'∙'"),
            Self::Cross => write!(f, "'×'"),
            Self::Slash => write!(f, "'/'"),
            Self::Divide => write!(f, "'÷'"),
            Self::Percent => write!(f, "'%'"),
            Self::Carrot => write!(f, "'^'"),
            Self::Sqrt => write!(f, "'√'"),
            Self::Cbrt => write!(f, "'∛'"),
            Self::Fort => write!(f, "'∜'"),
            Self::Degree => write!(f, "'°'"),
            Self::Exclamation => write!(f, "'!'"),
            Self::LeftParen => write!(f, "'('"),
            Self::RightParen => write!(f, "')'"),
            Self::LeftBrace => write!(f, "'{{'"),
            Self::RightBrace => write!(f, "'}}'"),
            Self::Pipe => write!(f, "'|'"),
            Self::LeftFloor => write!(f, "'⌊'"),
            Self::RightFloor => write!(f, "'⌋'"),
            Self::LeftCeil => write!(f, "'⌈'"),
            Self::RightCeil => write!(f, "'⌉'"),
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
