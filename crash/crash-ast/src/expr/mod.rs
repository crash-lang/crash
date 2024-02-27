use crate::expr::variable::VariableExpr;

mod variable;

#[derive(Clone)]
pub enum Expr {
     Var(VariableExpr),
    
}