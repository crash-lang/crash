use crate::types::Type;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    val_typ: Type,
    index: u16
}

impl Variable {

    pub fn new(val_typ: Type, index: u16) -> Self {
        Self { val_typ, index }
    }

    pub fn val_typ(&self) -> &Type {
        &self.val_typ
    }
    pub fn index(&self) -> u16 {
        self.index
    }
}