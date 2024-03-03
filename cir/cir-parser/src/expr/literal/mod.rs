mod boolean;
mod string;
mod character;
mod integer;
mod float;
mod path;

use std::sync::{Arc, Mutex};
use crash_ast::expr::literal::LiteralExpr;
use crate::stream::TokenStream;

impl TokenStream {

    pub fn try_parse_literal_expr(mutex_stream: Arc<Mutex<Self>>) -> Option<LiteralExpr> {
        if let Some(string_literal) = Self::try_parse_string_literal_expr(mutex_stream) {
            return Some(LiteralExpr::String(string_literal));
        }
        
        if let Some(bool_literal) = Self::try_parse_boolean_literal_expr(mutex_stream.clone()) {
            return Some(LiteralExpr::Boolean(bool_literal));
        }

        if let Some(integer_literal) = Self::try_parse_integer_literal_expr(mutex_stream.clone()) {
            return Some(LiteralExpr::Integer(integer_literal));
        }

        if let Some(float_literal) = Self::try_parse_float_literal_expr(mutex_stream.clone()) {
            return Some(LiteralExpr::Float(float_literal));
        }
        
        if let Some(char_literal) = Self::try_parse_char_literal_expr(mutex_stream.clone()) {
            return Some(LiteralExpr::Char(char_literal));
        }
        
        if let Some(path_literal) = Self::try_parse_path_literal_expr(mutex_stream.clone()) {
            return Some(LiteralExpr::Path(path_literal));
        }
        
        None
    }
}