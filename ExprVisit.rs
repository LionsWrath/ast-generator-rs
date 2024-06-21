use crate::ast::*;
use crate::token::Token;

pub trait ExprVisitor<T> {
    fn visit_expr(&mut self, e: &Expr) -> T;
    fn visit_comma(&mut self, c: &Comma) -> T;
    fn visit_ternary(&mut self, t: &Ternary) -> T;
    fn visit_binary(&mut self, b: &Binary) -> T;
    fn visit_grouping(&mut self, g: &Grouping) -> T;
    fn visit_literal(&mut self, l: &Literal) -> T;
    fn visit_unary(&mut self, u: &Unary) -> T;
}

