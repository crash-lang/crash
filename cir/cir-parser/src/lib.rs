use std::fs::File;
use std::io::Read;
use std::sync::{Arc, Mutex};
use crash_ast::function::Function;
use crash_ast::header::Header;
use crash_ast::types::UDT;
use crate::stream::TokenStream;

pub mod error;
pub(crate) mod structure;

mod stream;
mod header;
mod expr;
mod misc;

pub struct Parser {
    headers: Vec<Header>,
    types: Vec<UDT>,
    functions: Vec<Function>,
    file: File,
    children: Vec<Parser>
}

#[derive(Debug)]
enum Error {
    FileReadError(std::io::Error),
    IncludeFileOpenError { path: String, error: std::io::Error }
}

pub fn parse(mut root_file: File) -> Result<Parser, Error> {
    let mut content = String::new();

    match root_file.read_to_string(&mut content) {
        Err(err) => return Err(Error::FileReadError(err)),
        _ => {}
    }

    let mut parser = Parser {
        headers: Vec::new(),
        types: Vec::new(),
        functions: Vec::new(),
        file: root_file,
        children: Vec::new(),
    };

    let mutex_stream = Arc::new(Mutex::new(TokenStream::new(content)));
    
    loop {
        let stream = mutex_stream.lock().unwrap();
        if !stream.has_more_tokens() {
            break
        }

        drop(stream);

        if let Some(header) = TokenStream::try_parse_header(mutex_stream.clone()) {
            parser.headers.push(header);
        }
    }

    for header in parser.headers.clone() {

        if let Header::Include(include) = header {
            let path = include.path();
            let file = match File::open(path) {
                Ok(file) => file,
                Err(err) => {
                    return Err(Error::IncludeFileOpenError {
                        path: path.to_string(),
                        error: err
                    })
                }
            };
            parser.children.push(match parse(file) {
                Ok(parser) => parser,
                Err(err) => {
                    return Err(err);
                }
            });
        }


    }
    
    Ok(parser)
}