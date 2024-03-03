
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StringLiteralExpr {
    content: String
}

impl StringLiteralExpr {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}