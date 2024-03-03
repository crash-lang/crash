use crate::literals::path::PathLiteral;
use crate::stream::TokenStream;

mod path;

pub enum Literal {
    Path(PathLiteral<'static>)
}

impl TokenStream {
    
    pub fn try_parse_literal(&mut self) -> Option<Literal> {
        if let Some(path_literal) = self.try_parse_path() {
            return Some(Literal::Path(path_literal));
        }
        
        None
    }
    
    pub fn expect_literal(&mut self) -> Literal {
        match self.try_parse_literal() {
            None => Self::err_text(self.current(), "Expected literal"),
            Some(literal) => literal
        }
    }
}