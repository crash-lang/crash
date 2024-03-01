use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;
use crash_ast::AST;
use crash_ast::function::Function;
use crash_ast::types::UDT;
use crash_utils::files::diff_files;
use crate::stream::TokenStream;

pub struct ParserOld {
    types: Vec<UDT>,
    functions: Vec<Function>,
    file: File,
    stream: TokenStream,
    children: Vec<ParserOld>
}

impl ParserOld {

    pub fn parse(mut root_file: File) -> Self {
        let mut content = String::new();

        match root_file.read_to_string(&mut content) {
            Err(err) => {
                panic!("Unable to read file: {:?}", err)
            }
            _ => {}
        }

        let mut parser = ParserOld {
            types: Vec::new(),
            functions: Vec::new(),
            stream: TokenStream::new(content),
            file: root_file,
            children: Vec::new()
        };
        
        
        loop {
            if !parser.has_more_tokens() {
                break
            }

            match parser.try_parse_include() {
                Some(include_path) => {
                    let path = Path::new(include_path.path());
                    match File::open(path) {
                        Ok(file) => {
                            let mut child_parser = Self::parse(file);

                            if diff_files(&mut child_parser.file, &mut parser.file) {
                                println!("Include cycle in {:?}", parser.file);
                                exit(1);
                            }

                            parser.functions.append(&mut child_parser.functions);
                            parser.types.append(&mut child_parser.types);

                            parser.children.push(child_parser);
                        }
                        Err(err) => {
                            panic!("Unable to include file: {:?}", err)
                        }
                    }
                    continue
                }
                _ => {}
            }

        }

        parser
    }
    pub fn to_ast(self) -> AST {
        AST::new(self.types, self.functions)
    }

    pub fn types(&self) -> &Vec<UDT> {
        &self.types
    }

    pub fn functions(&self) -> &Vec<Function> {
        &self.functions
    }
}