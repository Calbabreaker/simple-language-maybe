use std::collections::HashMap;

use crate::{
    lexer::{Literal, Operator},
    parser::{Expr, ExprKind},
    Error, ErrorKind, Parser,
};

#[derive(Debug, Clone)]
pub enum Value<'a> {
    String(String),
    Number(f64),
    Reference(&'a Value<'a>),
    Boolean(bool),
}

#[derive(Debug)]
pub struct Variable<'a> {
    value: Value<'a>,
    constant: bool,
}

#[derive(Default)]
pub struct Intepreter<'a> {
    pub variables: HashMap<&'a str, Variable<'a>>,
}

impl<'a> Intepreter<'a> {
    pub fn run(&mut self, parser: &Parser<'a>) -> crate::Result<()> {
        for expr in &parser.tree {
            self.run_expr(expr)?;
        }

        Ok(())
    }

    fn run_expr(&mut self, expr: &Expr<'a>) -> crate::Result<()> {
        match expr.kind.as_ref() {
            ExprKind::Assignment {
                operator,
                left,
                right,
            } => {
                let name = match left.kind.as_ref() {
                    ExprKind::Identifier(identifier) => identifier,
                    _ => Err(Error::new(ErrorKind::RValueAssign, left.position))?,
                };

                let value = match right.kind.as_ref() {
                    ExprKind::Identifier(identifier) => match self.variables.get(identifier) {
                        Some(variable) => variable.value.clone(),
                        None => Err(Error::new(
                            ErrorKind::NameError(identifier.to_string()),
                            right.position,
                        ))?,
                    },
                    ExprKind::Literal(literal) => match literal {
                        Literal::True => Value::Boolean(true),
                        Literal::False => Value::Boolean(false),
                        Literal::Float(float) => Value::Number(*float),
                        Literal::Integer(int) => Value::Number(*int as f64),
                        Literal::String(str) => Value::String(str.to_string()),
                    },
                    _ => unimplemented!(),
                };

                let constant = match operator {
                    Operator::Equal => false,
                    Operator::ConstantEqual => true,
                    _ => unreachable!(),
                };

                if let Some(variable) = self.variables.get(name) {
                    if variable.constant {
                        Err(Error::new(ErrorKind::ConstantAssign, left.position))?
                    }
                }

                self.variables.insert(name, Variable { value, constant });
            }
            _ => unimplemented!(),
        }

        Ok(())
    }
}
