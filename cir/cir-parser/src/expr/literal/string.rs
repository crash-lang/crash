use crash_ast::expr::literal::StringLiteralExpr;
use crash_ir_lexer::TokenType;
use crate::stream::TokenStream;


impl TokenStream {
    
    pub fn try_parse_string_literal_expr(&mut self) -> Option<StringLiteralExpr> {
        match self.try_token(TokenType::StringLiteral) {
            None => None,
            Some(tok) => {
                let string_val = {
                    let val = tok.value();
                    let mut chars = val.chars();
                    chars.next();
                    chars.next_back();
                    chars.as_str()
                }.to_string();
                
                Some(StringLiteralExpr::new(string_val))
            }
        }
    }
}