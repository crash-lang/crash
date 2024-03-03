use std::sync::{Arc, Mutex};
use crash_ir_lexer::TokenType;
use crate::stream::TokenStream;

impl TokenStream {
    
    pub fn try_parse_identifier(mutex_stream: Arc<Mutex<Self>>) -> Option<String> {
        match Self::try_token(mutex_stream, TokenType::Identifier) {
            None => None,
            Some(tok) => {
                Some(tok.value().to_string())
            }
        }
    }
}