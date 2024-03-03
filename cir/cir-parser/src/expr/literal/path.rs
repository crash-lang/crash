use std::path::Path;
use std::sync::{Arc, Mutex};
use crash_ast::expr::literal::PathLiteralExpr;
use crash_ir_lexer::TokenType;
use crate::stream::TokenStream;

impl TokenStream {
    
    pub fn try_parse_path_literal_expr(mutex_stream: Arc<Mutex<Self>>) -> Option<PathLiteralExpr> {
        match Self::try_token(mutex_stream, TokenType::PathLiteral) {
            None => None,
            Some(tok) => {
                let path_val = {
                    let string_val = tok.value();
                    
                    Path::new(string_val)
                };
                
                Some(PathLiteralExpr::new(path_val))
            }
        }
    }
    
    pub fn expect_path_literal_expr(mutex_stream: Arc<Mutex<Self>>) -> PathLiteralExpr {
        let tok = Self::expect_token(mutex_stream, TokenType::PathLiteral);
        
        PathLiteralExpr::new(Path::new(tok.value()))
    }
}