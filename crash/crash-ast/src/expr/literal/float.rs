
#[derive(Clone, Debug, PartialEq)]
pub struct FloatLiteralExpr {
    value: FloatLiteralExprValue
}

#[derive(Clone, Debug, PartialEq)]
pub enum FloatLiteralExprValue {
    F32(f32),
    F64(f64)
}

impl FloatLiteralExpr {
    pub fn new(value: FloatLiteralExprValue) -> Self {
        Self { value }
    }
    
    pub fn value(&self) -> &FloatLiteralExprValue {
        &self.value
    }
}