use crate::expr::Expr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus
}

#[derive(Clone)]
pub struct ArithmeticExpr {
    operator: Operator,
    left: Expr,
    right: Expr
}

impl ArithmeticExpr {
    pub fn new(operator: Operator, left: Expr, right: Expr) -> Self {
        Self { operator, left, right }
    }
    
    pub fn operator(&self) -> &Operator {
        &self.operator
    }
    pub fn left(&self) -> &Expr {
        &self.left
    }
    pub fn right(&self) -> &Expr {
        &self.right
    }
}
