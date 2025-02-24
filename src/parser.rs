use crate::{AmiError, BinaryOp, Node, Token, TokenType};
use std::vec::IntoIter;

use TokenType::*;

pub struct Parser {
    tokens: IntoIter<Token>,
    token: Token,
}

type ParseResult = Result<Node, AmiError>;

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut iter = tokens.into_iter();
        Self {
            token: iter.next().unwrap_or(Token {
                ty: EOF,
                range: Default::default(),
            }),
            tokens: iter,
        }
    }

    fn advance(&mut self) {
        self.token = self.tokens.next().unwrap_or(Token {
            ty: EOF,
            range: Default::default(),
        });
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
            if statement == Node::EOF {
                more_statements = false;
                continue;
            }
            statements.push(statement);
        }

        Ok(Node::Statements(statements))
    }

    pub fn statement(&mut self) -> ParseResult {
        self.expr()
    }

    fn expr(&mut self) -> ParseResult {
        self.arith_expr()
    }

    fn arith_expr(&mut self) -> ParseResult {
        let result = self.atom()?;

        Ok(match self.token.ty {
            Plus => {
                self.advance();
                Node::Binary(
                    Box::new(result),
                    BinaryOp::Add,
                    Box::new(self.arith_expr()?),
                )
            }
            Minus => {
                self.advance();
                Node::Binary(
                    Box::new(result),
                    BinaryOp::Sub,
                    Box::new(self.arith_expr()?),
                )
            }
            _ => result,
        })
    }

    fn atom(&mut self) -> ParseResult {
        let start = self.token.range.start;

        match self.token.ty.clone() {
            Number(x) => {
                self.advance();
                Ok(Node::Number(x))
            }
            EOF => Ok(Node::EOF),
            _ => self.error(
                "expected token".to_string(),
                "expected number".to_string(),
                start,
            ),
        }
    }
}
