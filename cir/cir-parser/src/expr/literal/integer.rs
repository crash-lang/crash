use crash_ast::expr::literal::IntegerLiteralExpr;
use crash_ir_lexer::TokenType;
use crate::stream::TokenStream;

impl TokenStream {
    
    pub fn try_parse_integer_literal_expr(&mut self) -> Option<IntegerLiteralExpr> {
        match self.try_token(TokenType::IntegerLiteral) {
            None => None,
            Some(tok) => {
                
                
                None
            }
        }
    }
    
}