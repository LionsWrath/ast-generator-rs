use crate::token::Token;

#[derive(Clone, PartialEq, Debug)]
pub enum Stmt {
    EXPRESSION(Expression),
    PRINT(Print),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Expression {
    pub expr: Box<Expr>,
}

impl Expression {
   pub fn new(expr: Box<Expr>) -> Self {
        Expression {
            expr,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Print {
    pub expr: Box<Expr>,
}

impl Print {
   pub fn new(expr: Box<Expr>) -> Self {
        Print {
            expr,
        }
    }
}

