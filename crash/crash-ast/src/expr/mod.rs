use crate::expr::literal::LiteralExpr;

pub mod literal;

#[derive(Clone)]
pub enum Expr {
     Literal(LiteralExpr)
    
}