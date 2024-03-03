

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CharLiteralExpr {
    value: char
}

impl CharLiteralExpr {
    pub fn new(value: char) -> Self {
        Self { value }
    }
}