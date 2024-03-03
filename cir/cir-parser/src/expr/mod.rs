mod literal;

use std::sync::{Arc, Mutex};
use crash_ast::expr::Expr;
use crate::stream::TokenStream;

impl TokenStream {
    
    pub fn try_parse_expr(mutex_stream: Arc<Mutex<Self>>) -> Option<Expr> {
        if let Some(literal_expr) = Self::try_parse_literal_expr(mutex_stream) {
            return Some(Expr::Literal(literal_expr));
        }
        
        None
    }
}