use crash_ir_lexer::TokenType;
use crate::stream::TokenStream;
use crate::tok_struct;

tok_struct!(
    Constant {
        
    }
);

impl TokenStream {
    
    pub fn try_parse_constant(&mut self) -> Option<Constant> {
        match self.try_token(TokenType::Const) {
            Some(tok) => {
                let mut tokens = vec![tok];
                let start_pos = tok.pos().start();
                
                //TODO 
                
                let semicolon_tok = self.expect_token(TokenType::Semicolon);
                tokens.push(semicolon_tok.clone());
                
                let end_pos = semicolon_tok.pos().end();
                
                return Some(Constant {
                    tokens,
                    position: TokenPosition::new(
                        start_pos,
                        end_pos
                    ),
                });
            },
            None => None
        }
    }
    
}