use crate::expr::arithmetic::ArithmeticExpr;
use crate::expr::literal::LiteralExpr;
use crate::expr::variable::VariableExpr;

pub mod literal;
pub mod variable;
mod arithmetic;

#[derive(Clone)]
pub enum Expr {
     Literal(LiteralExpr),
     Variable(VariableExpr),
     Arithmetic(ArithmeticExpr),
     
}