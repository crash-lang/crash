mod literal;

use crash_ast::expr::Expr;
use crate::stream::TokenStream;

impl TokenStream {
    
    
    pub fn try_parse_expr(&mut self) -> Option<Expr> {
        if let Some(literal_expr) = self.try_parse_literal_expr() {
            //todo
        }
        
        
        None
    }
    
}