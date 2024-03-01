use std::fs::File;
use std::io::Read;
use crash_ast::function::Function;
use crash_ast::types::UDT;
use crate::headers::Header;
use crate::stream::TokenStream;

pub mod error;
pub(crate) mod structure;

mod parser;
mod includes;
mod literals;
mod instructions;
mod stream;
mod headers;

pub struct Parser {
    headers: Vec<Header>,
    types: Vec<UDT>,
    functions: Vec<Function>,
    file: File,
    children: Vec<Parser>
}

impl Parser {
    pub fn parse(mut root_file: File) -> Self {
        let mut content = String::new();

        match root_file.read_to_string(&mut content) {
            Err(err) => {
                panic!("Unable to read file: {:?}", err);
            }
            _ => {}
        }

        let mut parser = Parser {
            headers: Vec::new(),
            types: Vec::new(),
            functions: Vec::new(),
            file: root_file,
            children: Vec::new(),
        };
        
        let mut stream = TokenStream::new(content);

        loop {
            if !stream.has_more_tokens() {
                break
            }
            
            if let Some(mut header) = stream.try_parse_headers() {
                parser.headers.append(&mut header);
            }
            
            
        }

        parser
    }
}