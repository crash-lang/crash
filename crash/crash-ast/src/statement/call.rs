use crate::expr::Expr;

#[derive(Clone)]
pub struct FuncCall {
    fn_name: String,
    args: Vec<Expr>,
    
}