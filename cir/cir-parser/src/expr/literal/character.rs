use std::sync::{Arc, Mutex};
use crash_ast::expr::literal::CharLiteralExpr;
use crash_ir_lexer::TokenType;
use crate::stream::TokenStream;

impl TokenStream {
    
    pub fn try_parse_char_literal_expr(mutex_stream: Arc<Mutex<Self>>) -> Option<CharLiteralExpr> {
        match Self::try_token(mutex_stream, TokenType::CharLiteral) {
            None => None,
            Some(tok) => {
                let char_val = {
                    let mut val = tok.value();
                    let mut chars = val.chars();
                    chars.next();
                    chars.next_back();

                    if val.to_string().len() > 1 {
                        Self::err_text(tok, &format!("Invalid char literal {}", val));
                    }
                    
                    let char = match chars.next() {
                        None => Self::err_text(tok, "Invalid char literal"),
                        Some(char) => char
                    };
                    
                    char
                };
                
                Some(CharLiteralExpr::new(char_val))
            }
        }
    }
    
}