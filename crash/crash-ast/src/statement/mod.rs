pub use {
    crate::statement::call::FuncCall
};
use crate::statement::variable::Variable;

mod call;
mod variable;

#[derive(Clone)]
pub enum Statement {
    Call(FuncCall),
    Variable(Variable),
    
}