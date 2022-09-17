use std::{iter::Peekable, slice::Iter};

use crate::{
    lexer::{Literal, Operator},
    Error, ErrorKind, Lexer, Position, Token,
};

#[derive(Debug)]
pub enum ExprKind<'a> {
    Assignment {
        operator: Operator,
        left: Expr<'a>,
        right: Expr<'a>,
    },
    Binary {
        operator: Operator,
        left: Expr<'a>,
        right: Expr<'a>,
    },
    Call {
        callee: Expr<'a>,
        arguments: Vec<Expr<'a>>,
    },
    Identifier(&'a str),
    Literal(Literal<'a>),
}

#[derive(Debug)]
pub struct Expr<'a> {
    pub kind: Box<ExprKind<'a>>,
    pub position: Position,
}

pub struct Parser<'a> {
    pub tree: Vec<Expr<'a>>,
    tokens: Peekable<Iter<'a, (Token<'a>, Position)>>,
    position: Position,
}

impl<'a> Parser<'a> {
    pub fn parse(lexer: &'a Lexer) -> crate::Result<Self> {
        let mut parser = Parser {
            tree: Vec::new(),
            tokens: lexer.tokens.iter().peekable(),
            position: Position::default(),
        };

        while parser.peek_token() != Token::EOF {
            let expr = parser
                .expr_line()
                .map_err(|kind| Error::new(kind, parser.position))?;

            if parser.next_token() != Token::LineBreak {
                Err(Error::new(
                    ErrorKind::ExpectedToken("new line".to_string()),
                    parser.position,
                ))?
            }

            parser.tree.push(expr);
        }

        Ok(parser)
    }

    fn expr_line(&mut self) -> Result<Expr<'a>, ErrorKind> {
        let lexpr = self.expr()?;

        let expr = match self.next_token() {
            Token::Operator(operator) => match operator {
                Operator::Equal | Operator::ConstantEqual => self.assignment(lexpr, operator)?,
                Operator::Plus => unimplemented!(),
            },
            _ => unimplemented!(),
        };

        Ok(expr)
    }

    fn expr(&mut self) -> Result<Expr<'a>, ErrorKind> {
        let kind = match self.next_token() {
            Token::Identifier(identifier) => ExprKind::Identifier(identifier),
            Token::Literal(literal) => ExprKind::Literal(literal),
            _ => unimplemented!(),
        };

        Ok(Expr {
            kind: Box::new(kind),
            position: self.position,
        })
    }

    fn assignment(&mut self, lexpr: Expr<'a>, operator: Operator) -> Result<Expr<'a>, ErrorKind> {
        let rexpr = self.expr()?;
        let rpos = rexpr.position;
        let lpos = lexpr.position;
        Ok(Expr {
            kind: Box::new(ExprKind::Assignment {
                operator,
                left: lexpr,
                right: rexpr,
            }),
            position: Position::new(lpos.line, (lpos.columns.0, rpos.columns.1)),
        })
    }

    fn next_token(&mut self) -> Token<'a> {
        let (token, postion) = self.tokens.next().unwrap();
        self.position = *postion;
        *token
    }

    fn peek_token(&mut self) -> Token<'a> {
        let (token, _) = self.tokens.peek().unwrap();
        *token
    }
}
