pub use {
    crate::statement::call::FuncCall
};

mod call;

#[derive(Clone)]
pub enum Statement {
    Call(FuncCall),
    
}