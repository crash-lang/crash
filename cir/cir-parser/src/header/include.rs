use std::sync::{Arc, Mutex};
use crash_ast::header::Include;
use crash_ir_lexer::TokenType;
use crate::stream::TokenStream;

impl TokenStream {

    pub fn try_parse_include(mutex_stream: Arc<Mutex<Self>>) -> Option<Include> {
        match Self::try_token(mutex_stream.clone(), TokenType::Include) {
            None => None,
            Some(..) => {
                let expr = Self::expect_path_literal_expr(mutex_stream);
                Some(Include::new(expr.path()))
            }
        }
    }
}