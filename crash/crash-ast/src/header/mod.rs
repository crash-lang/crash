pub use {
    crate::header::include::*,
};

mod include;

#[derive(Clone)]
pub enum Header {
    Include(Include),
    Constant
}