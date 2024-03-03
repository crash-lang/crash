use std::fs::File;
use std::io::Read;
use tokio::runtime::Handle;
use crash_ast::function::Function;
use crash_ast::types::UDT;
use crate::headers::Header;
use crate::stream::TokenStream;

pub mod error;
pub(crate) mod structure;

mod literals;
mod instructions;
mod stream;
mod headers;
mod expr;

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

        let mut finished = 0;

        for header in parser.headers {
            
            if let Header::Include(include) = header {
                /* 
                    Doing it async for parallel parsing.
                    Analyser will check for mistakes later.
                 */
                Handle::current().spawn(async {
                    let path = include.path();
                    let file = match File::open(path) {
                        Ok(file) => file,
                        Err(err) => {
                            panic!("Unable to open file on path {:?}: {:?}", path, err);
                        }
                    };
                    parser.children.push(Parser::parse(file));
                    finished+=1;
                });
            }
            
            
        }

        // block until all files parsed
        while finished < parser.headers.len() {}

        parser
    }
}