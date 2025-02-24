use crate::{AmiError, BinaryOp, Node, NodeType, Token, TokenType, UnaryOp};
use std::{iter::Peekable, vec::IntoIter};

use TokenType::*;

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
    token: Token,
}

type ParseResult = Result<Node, AmiError>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut iter = tokens.into_iter().peekable();
        Self {
            token: iter.next().unwrap_or(Token {
                ty: EOF,
                range: Default::default(),
            }),
            tokens: iter,
        }
    }

    fn peek(&mut self) -> &TokenType {
        match self.tokens.peek() {
            Some(token) => &token.ty,
            None => &EOF,
        }
    }

    fn advance(&mut self) {
        self.token = self.tokens.next().unwrap_or(Token {
            ty: EOF,
            range: Default::default(),
        });
    }

    fn node(&self, ty: NodeType, start: usize) -> ParseResult {
        Ok(Node {
            ty,
            range: start..self.token.range.end,
        })
    }

    fn error<T>(&self, msg: String, reason: String, start: usize) -> Result<T, AmiError> {
        Err(AmiError {
            msg,
            reason,
            range: start..self.token.range.end,
        })
    }

    fn skip_newlines(&mut self) -> u32 {
        let mut newlines = 0u32;
        while self.token.ty == Newline {
            self.advance();
            newlines += 1;
        }
        newlines
    }

    pub fn parse(&mut self) -> ParseResult {
        self.statements()
    }

    fn statements(&mut self) -> ParseResult {
        let start = self.token.range.start;
        let mut statements: Vec<Node> = vec![];
        self.skip_newlines();

        statements.push(self.statement()?);

        let mut more_statements = true;

        loop {
            let newlines = self.skip_newlines();
            if newlines == 0 {
                more_statements = false;
            }

            if !more_statements {
                break;
            }

            let statement = self.statement()?;
            if statement.ty == NodeType::EOF {
                more_statements = false;
                continue;
            }
            statements.push(statement);
        }

        self.node(NodeType::Statements(statements), start)
    }

    pub fn statement(&mut self) -> ParseResult {
        self.expr()
    }

    fn expr(&mut self) -> ParseResult {
        let start = self.token.range.start;

        match (self.token.ty.clone(), self.peek()) {
            (Identifier(name), Eq) => {
                self.advance();
                self.advance();
                let right = self.arith_expr()?;
                self.node(NodeType::Assignment(name, Box::new(right)), start)
            }
            _ => self.arith_expr(),
        }
    }

    fn arith_expr(&mut self) -> ParseResult {
        let start = self.token.range.start;
        let left = self.term()?;

        match self.token.ty {
            Plus => {
                self.advance();
                let right = self.arith_expr()?;
                self.node(
                    NodeType::Binary(Box::new(left), BinaryOp::Add, Box::new(right)),
                    start,
                )
            }
            Minus => {
                self.advance();
                let right = self.arith_expr()?;
                self.node(
                    NodeType::Binary(Box::new(left), BinaryOp::Sub, Box::new(right)),
                    start,
                )
            }
            _ => Ok(left),
        }
    }

    fn term(&mut self) -> ParseResult {
        let start = self.token.range.start;
        let left = self.factor()?;

        match self.token.ty {
            Star | Dot | Cross => {
                self.advance();
                let right = self.term()?;
                self.node(
                    NodeType::Binary(Box::new(left), BinaryOp::Mul, Box::new(right)),
                    start,
                )
            }
            Slash | Divide => {
                self.advance();
                let right = self.term()?;
                self.node(
                    NodeType::Binary(Box::new(left), BinaryOp::Div, Box::new(right)),
                    start,
                )
            }
            Percent | Mod => {
                self.advance();
                let right = self.term()?;
                self.node(
                    NodeType::Binary(Box::new(left), BinaryOp::Mod, Box::new(right)),
                    start,
                )
            }
            _ => Ok(left),
        }
    }

    fn factor(&mut self) -> ParseResult {
        let start = self.token.range.start;

        match self.token.ty {
            Plus => {
                self.advance();
                let right = self.factor()?;
                self.node(NodeType::Unary(UnaryOp::Pos, Box::new(right)), start)
            }
            Minus => {
                self.advance();
                let right = self.factor()?;
                self.node(NodeType::Unary(UnaryOp::Neg, Box::new(right)), start)
            }
            _ => self.power(),
        }
    }

    fn power(&mut self) -> ParseResult {
        let start = self.token.range.start;
        let result = self.prefix()?;

        match self.token.ty {
            Carrot => {
                self.advance();
                let exponent = self.factor()?;
                self.node(
                    NodeType::Binary(Box::new(result), BinaryOp::Pow, Box::new(exponent)),
                    start,
                )
            }
            _ => Ok(result),
        }
    }

    fn prefix(&mut self) -> ParseResult {
        let start = self.token.range.start;

        match self.token.ty {
            Sqrt => {
                self.advance();
                let left = self.prefix()?;
                self.node(NodeType::Unary(UnaryOp::Sqrt, Box::new(left)), start)
            }
            Cbrt => {
                self.advance();
                let left = self.prefix()?;
                self.node(NodeType::Unary(UnaryOp::Cbrt, Box::new(left)), start)
            }
            Fort => {
                self.advance();
                let left = self.prefix()?;
                self.node(NodeType::Unary(UnaryOp::Fort, Box::new(left)), start)
            }
            _ => self.postfix(),
        }
    }

    fn postfix(&mut self) -> ParseResult {
        let start = self.token.range.start;
        let result = self.atom()?;

        match self.token.ty {
            Exclamation => {
                self.advance();
                self.node(NodeType::Unary(UnaryOp::Fact, Box::new(result)), start)
            }
            Degree => {
                self.advance();
                self.node(NodeType::Unary(UnaryOp::Degree, Box::new(result)), start)
            }
            _ => Ok(result),
        }
    }

    fn atom(&mut self) -> ParseResult {
        let start = self.token.range.start;

        match self.token.ty.clone() {
            Number(x) => {
                self.advance();
                self.node(NodeType::Number(x), start)
            }
            Identifier(name) => {
                self.advance();
                self.node(NodeType::Identifier(name), start)
            }
            LeftParen => {
                self.advance();
                let result = self.arith_expr()?;

                if self.token.ty != RightParen {
                    return self.error(
                        "expected token".to_string(),
                        format!("expected {}", RightParen),
                        start,
                    );
                }
                self.advance();

                Ok(result)
            }
            Pipe => {
                self.advance();
                let result = self.arith_expr()?;

                if self.token.ty != Pipe {
                    return self.error(
                        "expected token".to_string(),
                        format!("expected {}", Pipe),
                        start,
                    );
                }
                self.advance();

                self.node(NodeType::Unary(UnaryOp::Abs, Box::new(result)), start)
            }
            LeftFloor => {
                self.advance();
                let result = self.arith_expr()?;

                match self.token.ty {
                    RightFloor => {
                        self.advance();
                        self.node(NodeType::Unary(UnaryOp::Floor, Box::new(result)), start)
                    }
                    RightCeil => {
                        self.advance();
                        self.node(NodeType::Unary(UnaryOp::Abs, Box::new(result)), start)
                    }
                    _ => self.error(
                        "expected token".to_string(),
                        format!("expected {} or {}", RightFloor, RightCeil),
                        start,
                    ),
                }
            }
            LeftCeil => {
                self.advance();
                let result = self.arith_expr()?;

                if self.token.ty != RightCeil {
                    return self.error(
                        "expected token".to_string(),
                        format!("expected {}", RightCeil),
                        start,
                    );
                }
                self.advance();

                self.node(NodeType::Unary(UnaryOp::Ceil, Box::new(result)), start)
            }
            EOF => self.node(NodeType::EOF, start),
            _ => self.error(
                "expected token".to_string(),
                format!(
                    "expected number, variable, function name, {}, {}, {}, or {}",
                    LeftParen, Pipe, LeftFloor, LeftCeil
                ),
                start,
            ),
        }
    }
}
