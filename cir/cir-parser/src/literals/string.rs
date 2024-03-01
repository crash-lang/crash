use crash_ir_lexer::{TokenPosition, TokenType};
use crate::parser::ParserOld;

pub struct StringLiteral {
    position: TokenPosition,
    content: String
}

impl ParserOld {
    
    
    pub fn try_parse_string(&mut self) -> Option<StringLiteral> {
        let mut current_tok = self.current();
        
        if current_tok.tok_type() == TokenType::StringLiteral {
            let content = current_tok.value();
            
            
            
        }
        
        None
    } 
    
}