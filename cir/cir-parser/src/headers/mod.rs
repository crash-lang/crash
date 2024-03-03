mod include;
mod constants;

use crate::headers::constants::Constant;
use crate::headers::include::Include;
use crate::stream::TokenStream;

pub enum Header {
    Include(Include<'static>),
    Constant(Constant)
}

impl TokenStream {
    
    pub fn try_parse_headers(&mut self) -> Option<Vec<Header>> {
        let mut header = Vec::new();
        
        /* 
            it tries to parse every possible header statement
            until nothing similar to this is left anymore         
        */
        loop {
            if let Some(include) = self.try_parse_include() {
                header.push(Header::Include(include));
                continue
            }
            
            if let Some(constant) = self.try_parse_constant() {
                header.push(Header::Constant(constant));
                continue
            }
            
            break
        }
        
        if header.is_empty() {
            return None;
        }
        
        Some(header)
    }
}