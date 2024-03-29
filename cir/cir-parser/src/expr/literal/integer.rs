use std::sync::{Arc, Mutex};
use crash_ast::expr::literal::IntegerLiteralExpr;
use crash_ir_lexer::TokenType;
use crate::stream::TokenStream;

impl TokenStream {
    
    pub fn try_parse_integer_literal_expr(mutex_stream: Arc<Mutex<Self>>) -> Option<IntegerLiteralExpr> {
        match Self::try_token(mutex_stream, TokenType::IntegerLiteral) {
            None => None,
            Some(tok) => {
                //todo
                None
            }
        }
    }
    
}