mod boolean;
mod string;
mod character;
mod integer;

use crash_ast::expr::literal::LiteralExpr;
use crate::stream::TokenStream;

impl TokenStream {

    pub fn try_parse_literal_expr(&mut self) -> Option<LiteralExpr> {
        if let Some(string_literal) = self.try_parse_string_literal_expr() {
            return Some(LiteralExpr::String(string_literal));
        }
        
        if let Some(bool_literal) = self.try_parse_boolean_literal_expr() {
            return Some(LiteralExpr::Boolean(bool_literal));
        }
        
        if let Some(char_literal) = self.try_parse_char_literal_expr() {
            return Some(LiteralExpr::Char(char_literal));
        }
        
        
        
        
        None
    }
}