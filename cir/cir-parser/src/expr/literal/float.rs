use std::sync::{Arc, Mutex};
use crash_ast::expr::literal::FloatLiteralExpr;
use crash_ir_lexer::TokenType;
use crate::stream::TokenStream;

impl TokenStream {
    
    pub fn try_parse_float_literal_expr(mutex_stream: Arc<Mutex<Self>>) -> Option<FloatLiteralExpr> {
        match Self::try_token(mutex_stream, TokenType::FloatLiteral) {
            None => None,
            Some(tok) => {
                //todo
                None
            }
        }
    }
}