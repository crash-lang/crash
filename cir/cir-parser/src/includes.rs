use tokio::sync::Mutex;
use crash_ir_lexer::TokenType;
use crate::parser::Parser;

impl Parser {

    pub fn try_parse_include(&mut self) -> Option<String> {
        let mut current_tok = self.current();
        let mut current_type = current_tok.tok_type();

        if current_type == TokenType::Include {
            let start_pos = current_tok.position();

            self.advance();



        }

        None
    }
}