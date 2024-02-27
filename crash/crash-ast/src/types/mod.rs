pub use crate::types::udt::UDT;

pub(crate) mod udt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Type {
    Void,
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    U128,
    I128,
    F32,
    F64,
    Char,
    Bool,

    UDT(UDT)
}