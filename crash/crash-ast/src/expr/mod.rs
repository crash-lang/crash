use crate::expr::literal::LiteralExpr;
use crate::variable::Variable;

mod literal;

pub enum Expr {
    Literal(LiteralExpr),
    Pointer(Box<Expr>),
    Var(Variable)
}