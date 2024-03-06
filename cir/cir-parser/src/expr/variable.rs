use std::sync::{Arc, Mutex};
use crash_ast::expr::variable::VariableExpr;
use crash_ir_lexer::TokenType;
use crate::stream::TokenStream;

impl TokenStream {
    
    pub fn try_parse_variable_expr(mutex_stream: Arc<Mutex<Self>>) -> Option<VariableExpr> {        
        match Self::try_token(mutex_stream, TokenType::Identifier) {
            None => None,
            Some(tok) => Some(VariableExpr::new(tok.value().to_string()))
        }
    }
    
}