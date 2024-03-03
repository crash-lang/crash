use std::sync::{Arc, Mutex};
use crash_ast::expr::literal::BooleanLiteralExpr;
use crash_ir_lexer::TokenType;
use crate::stream::TokenStream;

impl TokenStream {

    pub fn try_parse_boolean_literal_expr(mutex_stream: Arc<Mutex<Self>>) -> Option<BooleanLiteralExpr> {
        match Self::try_token(mutex_stream, TokenType::BooleanLiteral) {
            None => None,
            Some(tok) => {
                let clone = tok.clone();
                let val = clone.value();
                
                let bool_val = match val { 
                    "true" => true,
                    "false" => false,
                    _ => {
                        Self::err_text(tok, &format!("Invalid boolean literal {}", val))
                    }
                };
                
                Some(BooleanLiteralExpr::new(bool_val))
            }
        }
    }
}