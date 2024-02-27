use crate::instructions::Instruction;
use crate::types::Type;

#[derive(Clone)]
pub struct Function {
    name: String,
    return_type: Type,
    instructions: Vec<Instruction>
}

impl Function {
    pub fn new(name: String, return_type: Type, instructions: Vec<Instruction>) -> Self {
        Self { name, return_type, instructions }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn return_type(&self) -> &Type {
        &self.return_type
    }
    pub fn instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }
}