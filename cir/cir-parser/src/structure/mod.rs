mod macros;
use crash_ir_lexer::{Token, TokenPosition};

pub trait StructureInfo {
    
    fn tokens(self) -> Vec<Token>;
    
    fn position(&self) -> TokenPosition;
}
