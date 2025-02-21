use crate::{Node, Token};
use std::vec::IntoIter;
use Token::*;

pub struct Parser {
    tokens: IntoIter<Token>,
    token: Token,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut iter = tokens.into_iter();
        Self {
            token: iter.next().unwrap_or(EOF),
            tokens: iter,
        }
    }

    fn advance(&mut self) {
        self.token = self.tokens.next().unwrap_or(EOF);
    }

    fn skip_newlines(&mut self) -> u32 {
        let mut newlines = 0u32;
        while self.token == Newline {
            self.advance();
            newlines += 1;
        }
        newlines
    }

    pub fn parse(&mut self) -> Node {
        self.statements()
    }

    fn statements(&mut self) -> Node {
        let mut statements: Vec<Node> = vec![];
        self.skip_newlines();

        statements.push(self.statement());

        let mut more_statements = true;

        loop {
            let newlines = self.skip_newlines();
            if newlines == 0 {
                more_statements = false;
            }

            if !more_statements {
                break;
            }

            let statement = self.statement();
            if statement == Node::EOF {
                more_statements = false;
                continue;
            }
            statements.push(statement);
        }

        Node::Statements(statements)
    }

    pub fn statement(&mut self) -> Node {
        self.expr()
    }

    fn expr(&mut self) -> Node {
        self.atom()
    }

    fn atom(&mut self) -> Node {
        match self.token.clone() {
            Number(x) => {
                self.advance();
                Node::Number(x)
            }
            EOF => Node::EOF,
            _ => panic!("expected number"),
        }
    }
}
