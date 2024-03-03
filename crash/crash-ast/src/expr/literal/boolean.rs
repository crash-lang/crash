
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BooleanLiteralExpr {
    value: bool
}

impl BooleanLiteralExpr {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
    
    pub fn value(&self) -> bool {
        self.value
    }
}