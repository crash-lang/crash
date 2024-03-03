mod include;

use std::sync::{Arc, Mutex};
use crash_ast::header::Header;
use crate::stream::TokenStream;

impl TokenStream {
    
    pub fn try_parse_header(mutex_stream: Arc<Mutex<Self>>) -> Option<Header> {
        if let Some(include) = Self::try_parse_include(mutex_stream.clone()) {
            return Some(Header::Include(include));
        }
        
        None
    }
}