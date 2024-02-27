use crate::expr::Expr;
use crate::types::Type;
use crate::var::Variable;

#[derive(Clone)]
pub struct MoveInstruction {
    target: Variable,
    value: Expr
}

impl MoveInstruction {
    
    pub fn var_type(&self) -> &Type {
        &self.target.val_typ()
    }
    
    pub fn target(&self) -> &Variable {
        &self.target
    }
    pub fn value(&self) -> &Expr {
        &self.value
    }
}