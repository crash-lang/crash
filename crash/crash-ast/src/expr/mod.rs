use crate::expr::variable::VariableExpr;

mod variable;
pub mod literal;

#[derive(Clone)]
pub enum Expr {
     Var(VariableExpr),
    
}