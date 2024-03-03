use crate::types::primitive::PrimitiveType;

mod primitive;

#[derive(Clone)]
pub enum String {
    Primitive(PrimitiveType),
    
    
}