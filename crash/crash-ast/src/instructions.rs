use crate::expr::Expr;
use crate::identifier::Identifier;
use crate::types::Type;

pub enum Instructions {
    Set(Expr, Type),
    Move(Expr, Expr),
    Drop(Expr),
    Call(Identifier),
    Reference(Expr)
}

impl Instructions {
    
    pub fn as_str(&self) -> &str {
        match self {
            Instructions::Set {..} => "set",
            Instructions::Move{..} => "mov",
            Instructions::Drop{..} => "drop",
            Instructions::Call{..} => "call",
            Instructions::Reference{..} => "ref"
        }
    }
}