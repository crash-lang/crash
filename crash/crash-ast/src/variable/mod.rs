use crate::types::Type;

pub enum VarType {
    /// Used to declare function parameters
    Argument(Type),
    /// Used to initialize function parameters
    /// before the function gets called
    Parameter(),
    /// Variables inside structs
    Field,
    /// Variables inside functions
    Local
}

pub struct Variable {
    var_type: VarType,
    index: u32
}