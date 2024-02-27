use std::process::exit;
use crash_ir_lexer::{TokenPosition, TokenType};
use crate::parser::Parser;

pub struct Name {
    position: TokenPosition,
    name: String
}

impl Parser {

    pub fn expect_name(&mut self) -> Name {
        let mut current_tok = self.current();

        if current_tok.tok_type() == TokenType::Name {
            let content = current_tok.value().to_string();

            self.advance();

            return Name {
                position: current_tok.position(),
                name: content
            };
        }

        println!("Expecting name at {:?}", current_tok.position());

        exit(1);
    }
}

impl Name {
    pub fn position(&self) -> TokenPosition {
        self.position
    }
    pub fn name(self) -> String {
        self.name
    }
}