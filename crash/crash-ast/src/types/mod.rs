use crate::types::primitive::PrimitiveType;
use crate::types::udt::Struct;

mod primitive;
mod udt;

#[derive(Clone)]
pub enum Type {
    Primitive(PrimitiveType),
    UDT(UDT)
}

#[derive(Clone)]
pub enum UDT {
    Struct(Struct),
}