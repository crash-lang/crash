use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;
use crash_ast::AST;
use crash_ast::function::Function;
use crash_ast::types::UDT;
use crash_ir_lexer::{Token, tokenize, TokenPosition, TokenType};
use crash_utils::files::diff_files;

pub struct Parser {
    types: Vec<UDT>,
    functions: Vec<Function>,
    content: String,
    file: File,
    tokens: Vec<Token>,
    index: usize,
    children: Vec<Parser>
}

impl Parser {

    pub fn parse(mut root_file: File) -> Self {
        let mut content = String::new();

        match root_file.read_to_string(&mut content) {
            Err(err) => {
                panic!("Unable to read file: {:?}", err)
            }
            _ => {}
        }

        let mut parser = Parser {
            types: Vec::new(),
            functions: Vec::new(),
            tokens: Vec::new(),
            content,
            index: 0,
            file: root_file,
            children: Vec::new()
        };

        parser.tokenize();

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

    pub fn expect_token(&mut self, tok_type: TokenType) -> Token {
        let current_tok = self.current();

        if current_tok.tok_type() == tok_type {
            self.advance();

            return current_tok;
        }

        println!("Expected {:?} at {:?}", tok_type, current_tok.position());

        exit(1)
    }

    pub fn expect_type(&mut self, tok_type: TokenType) -> TokenPosition {
        self.expect_token(tok_type).position()
    }
    
    pub fn next_is_type(&self, tok_type: TokenType) -> bool {
        self.current().tok_type() == tok_type
    }

    pub(crate) fn tok_at_index(&self, index: usize) -> Token {
        match self.tokens.get(index) {
            None => Token::eof(),
            Some(tok) => tok.clone()
        }
    }

    pub(crate) fn current(&self) -> Token {
        self.tok_at_index(self.index)
    }

    pub(crate) fn prev(&self) -> Token {
        self.tok_at_index(self.index - 1)
    }

    pub(crate) fn peak(&self, amount: usize) -> Token {
        self.tok_at_index(self.index + amount)
    }

    pub(crate) fn advance(&mut self) {
        self.add(1)
    }

    pub(crate) fn reverse(&mut self) {
        self.remove(1)
    }

    pub(crate) fn add(&mut self, amount: usize) {
        self.index += amount
    }

    pub(crate) fn remove(&mut self, amount: usize) {
        self.index -= amount
    }

    fn tokenize(&mut self) {
        self.tokens = tokenize(self.content());
    }

    pub(crate) fn has_more_tokens(&self) -> bool {
        if self.tokens.len() <= self.index {
            return false;
        }

        match self.tokens.get(self.index) {
            None => false,
            Some(tok) => tok.tok_type() != TokenType::Eof
        }
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

    pub fn content(&self) -> &str {
        &self.content
    }
}