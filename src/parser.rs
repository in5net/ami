use crate::{AmiError, BinaryOp, Node, NodeType, Token, TokenType, UnaryOp};
use std::{iter::Peekable, rc::Rc, vec::IntoIter};

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

        if matches!(self.token.ty, Number(_))
            && !matches!(
                self.peek(),
                Number(_)
                    | Superscript(_)
                    | Plus
                    | Minus
                    | Star
                    | Dot
                    | Cross
                    | Slash
                    | Divide
                    | Percent
                    | Carrot
                    | RightParen
                    | LeftBrace
                    | RightBrace
                    | Pipe
                    | RightFloor
                    | RightCeil
                    | Newline
                    | EOF
            )
        {
            let left = self.atom()?;
            let right = self.term()?;
            return self.node(
                NodeType::Binary(Box::new(left), BinaryOp::Mul, Box::new(right)),
                start,
            );
        }

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
        let result = self.call()?;

        match self.token.ty.clone() {
            Exclamation => {
                self.advance();
                self.node(NodeType::Unary(UnaryOp::Fact, Box::new(result)), start)
            }
            Degree => {
                self.advance();
                self.node(NodeType::Unary(UnaryOp::Degree, Box::new(result)), start)
            }
            Superscript(tokens) => {
                self.advance();
                self.node(
                    NodeType::Binary(
                        Box::new(result),
                        BinaryOp::Pow,
                        Box::new(Parser::new(tokens).arith_expr()?),
                    ),
                    start,
                )
            }
            _ => Ok(result),
        }
    }

    fn call(&mut self) -> ParseResult {
        let start = self.token.range.start;
        let result = self.atom()?;

        match self.token.ty {
            LeftParen => {
                let list_start = self.token.range.start;
                let name = match result.ty {
                    NodeType::Identifier(ref name) => Rc::clone(name),
                    _ => panic!("expected identifier"),
                };
                self.advance();

                let args = self.list(list_start, RightParen)?;

                match self.token.ty {
                    Eq => {
                        self.advance();

                        let mut arg_names: Vec<Rc<str>> = vec![];
                        for node in args {
                            let arg = match node.ty {
                                NodeType::Identifier(name) => Ok(name),
                                _ => self.error(
                                    "expected identifier".to_string(),
                                    "".to_string(),
                                    start,
                                ),
                            }?;
                            arg_names.push(arg);
                        }

                        let body = self.expr()?;

                        self.node(NodeType::FnDef(name, arg_names, Box::new(body)), start)
                    }
                    _ => match result.ty {
                        NodeType::Identifier(name) => self.node(NodeType::Call(name, args), start),
                        _ => self.error(
                            "expected token".to_string(),
                            "there should be an identifier here".to_string(),
                            start,
                        ),
                    },
                }
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

    fn list(&mut self, start: usize, end: TokenType) -> Result<Vec<Node>, AmiError> {
        let mut nodes: Vec<Node> = vec![];

        while self.token.ty != end {
            nodes.push(self.expr()?);
            match &self.token.ty {
                Comma => self.advance(),
                token if *token == end => {}
                _ => {
                    return self.error(
                        "expected token".to_string(),
                        format!("expected {} or {}", Comma, end),
                        start,
                    )
                }
            };
        }

        if self.token.ty != end {
            return self.error(
                "expected token".to_string(),
                format!("expected {}", end),
                start,
            );
        }
        self.advance();

        Ok(nodes)
    }
}
