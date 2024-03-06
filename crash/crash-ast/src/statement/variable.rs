use crate::expr::Expr;
use crate::types::Type;

#[derive(Clone)]
pub struct Variable {
    variable_name: String,
    variable_type: Type,
    expr: Option<Expr>
}

impl Variable {
    pub fn new(variable_name: String, variable_type: Type, expr: Option<Expr>) -> Self {
        Self { variable_name, variable_type, expr }
    }
    
    pub fn variable_name(&self) -> &str {
        &self.variable_name
    }
    pub fn variable_type(&self) -> &Type {
        &self.variable_type
    }
    pub fn expr(&self) -> &Option<Expr> {
        &self.expr
    }
}