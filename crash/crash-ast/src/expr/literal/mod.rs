pub use {
    crate::expr::literal::character::*,
    crate::expr::literal::float::*,
    crate::expr::literal::integer::*,
    crate::expr::literal::string::*,
    crate::expr::literal::boolean::*
};

mod string;
mod integer;
mod float;
mod character;
mod boolean;

#[derive(Clone, PartialEq, Debug)]
pub enum LiteralExpr {
    String(StringLiteralExpr),
    Integer(IntegerLiteralExpr),
    Float(FloatLiteralExpr),
    Char(CharLiteralExpr),
    Boolean(BooleanLiteralExpr)
}