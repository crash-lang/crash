use crate::types::Type;
use crate::var::Variable;

#[derive(Clone)]
pub struct InitInstruction {
    target: Variable
}

impl InitInstruction {
    pub fn new(target: Variable) -> Self {
        Self { target }
    }

    pub fn val_type(&self) -> &Type {
        &self.target.val_typ()
    }
}