use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Number(String),
    Statements(Vec<Node>),
    EOF,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Number(x) => write!(f, "{}", x),
            Node::Statements(nodes) => write!(
                f,
                "{{\n  {}\n}}",
                nodes
                    .iter()
                    .map(|node| format!("{}", node))
                    .collect::<Vec<String>>()
                    .join("\n  ")
            ),
            Node::EOF => write!(f, "<eof>"),
        }
    }
}
