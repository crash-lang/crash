
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IntegerLiteralExpr {
    value: IntegerLiteralExprValue
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IntegerLiteralExprValue {
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    U128(u128),
    I128(i128)
}


impl IntegerLiteralExpr {
    pub fn new(value: IntegerLiteralExprValue) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &IntegerLiteralExprValue {
        &self.value
    }
}