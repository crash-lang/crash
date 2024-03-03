use crash_ast::expr::literal::BooleanLiteralExpr;
use crash_ir_lexer::TokenType;
use crate::stream::TokenStream;

impl TokenStream {

    pub fn try_parse_boolean_literal_expr(&mut self) -> Option<BooleanLiteralExpr> {
        match self.try_token(TokenType::BooleanLiteral) {
            None => None,
            Some(tok) => {
                let bool_val = {
                    let val = tok.value();
                    
                    if val == "true" {
                        true
                    } else if val == "false" {
                        false
                    } else {
                        Self::err_text(tok, &format!("Invalid boolean literal {}", val))
                    }
                };
                
                Some(BooleanLiteralExpr::new(bool_val))
            }
        }
    }
}