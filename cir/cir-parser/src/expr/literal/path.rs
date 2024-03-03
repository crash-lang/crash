use std::sync::{Arc, Mutex};
use crash_ast::expr::literal::PathLiteralExpr;
use crash_ir_lexer::TokenType;
use crate::stream::TokenStream;

impl TokenStream {

    pub fn try_parse_path_literal_expr(mutex_stream: Arc<Mutex<Self>>) -> Option<PathLiteralExpr> {
        match Self::try_token(mutex_stream, TokenType::PathLiteral) {
            None => None,
            Some(tok) => {                
                Some(PathLiteralExpr::new(tok.value().to_string()))
            }
        }
    }

    pub fn expect_path_literal_expr(mutex_stream: Arc<Mutex<Self>>) -> PathLiteralExpr {
        let tok = Self::expect_token(mutex_stream, TokenType::PathLiteral);

        PathLiteralExpr::new(tok.value().to_string())
    }
}