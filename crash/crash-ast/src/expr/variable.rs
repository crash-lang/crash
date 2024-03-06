
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VariableExpr {
    variable_name: String
}

impl VariableExpr {
    pub fn new(variable_name: String) -> Self {
        Self { variable_name }
    }
    
    pub fn variable_name(&self) -> &str {
        &self.variable_name
    }
}

