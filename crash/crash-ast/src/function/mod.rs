use crate::statement::Statement;
use crate::types::Type;

#[derive(Clone)]
pub struct Function {
    name: Type,
    statements: Vec<Statement>,
    return_type: Type
}

impl Function {


    
    pub fn statements(&self) -> &Vec<Statement> {
        &self.statements
    }
    
    pub fn name(&self) -> &Type {
        &self.name
    }
    
    pub fn return_type(&self) -> &Type {
        &self.return_type
    }
}