use std::{fmt, ops::Range, rc::Rc};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UnaryOp {
    Pos,
    Neg,
    Degree,
    Fact,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "×"),
            Self::Div => write!(f, "÷"),
            Self::Mod => write!(f, "mod"),
            Self::Pow => write!(f, "^"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Number(Rc<str>),
    Unary(UnaryOp, Box<Node>),
    Binary(Box<Node>, BinaryOp, Box<Node>),
    Statements(Vec<Node>),
    EOF,
}

impl fmt::Display for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(x) => write!(f, "{}", x),
            Self::Unary(op, node) => match op {
                UnaryOp::Pos => write!(f, "(+{})", node),
                UnaryOp::Neg => write!(f, "(-{})", node),
                UnaryOp::Degree => write!(f, "({}°)", node),
                UnaryOp::Fact => write!(f, "({}!)", node),
            },
            Self::Binary(left, op, right) => write!(f, "({} {} {})", left, op, right),
            Self::Statements(nodes) => write!(
                f,
                "{{\n  {}\n}}",
                nodes
                    .iter()
                    .map(|node| format!("{}", node))
                    .collect::<Vec<String>>()
                    .join("\n  ")
            ),
            Self::EOF => write!(f, "<eof>"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub ty: NodeType,
    pub range: Range<usize>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.ty)
    }
}
