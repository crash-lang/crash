use crate::statement::Statement;
use crate::types::String;

#[derive(Clone)]
pub struct Function {
    name: String,
    statements: Vec<Statement>,
    return_type: String
}

impl Function {


    
    pub fn statements(&self) -> &Vec<Statement> {
        &self.statements
    }
    
    pub fn name(&self) -> &String {
        &self.name
    }
    
    pub fn return_type(&self) -> &String {
        &self.return_type
    }
}