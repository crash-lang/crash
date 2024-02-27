use crate::var::Variable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableExpr(Variable);

impl VariableExpr {
    
    fn get_variable(self) -> Variable {
        self.0
    }
}