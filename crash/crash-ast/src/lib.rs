use crate::function::Function;
use crate::types::udt::UDT;

pub mod expr;
pub mod instructions;
pub mod var;
pub mod types;
pub mod function;
mod header;

pub struct AST {
    types: Vec<UDT>,
    functions: Vec<Function>
}

impl AST {
    pub fn new(types: Vec<UDT>, functions: Vec<Function>) -> Self {
        Self { types, functions }
    }

    pub fn types(&self) -> &Vec<UDT> {
        &self.types
    }

    pub fn functions(&self) -> &Vec<Function> {
        &self.functions
    }
}