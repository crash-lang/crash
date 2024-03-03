use crate::header::Header;

pub mod expr;
pub mod header;
pub mod function;
pub mod statement;
pub mod types;

pub struct AST {
    headers: Vec<Header>,

}

impl AST {
    
    
    pub fn headers(&self) -> &Vec<Header> {
        &self.headers
    }
}