use crash_ir_lexer::{TokenPosition, TokenType};
use crash_ir_lexer::TokenType::Semicolon;
use crate::parser::ParserOld;

pub struct Include {
    position: TokenPosition,
    path: String
}

impl ParserOld {

    pub fn try_parse_include(&mut self) -> Option<Include> {
        let current_tok = self.current();
        let current_type = current_tok.tok_type();

        if current_type == TokenType::Include {
            let start_pos = current_tok.position();
            self.advance();

            let name_tok = self.expect_token(TokenType::Name);
            let end_pos = self.expect_type(Semicolon);

            return Some(Include {
                position: TokenPosition::new(
                    *start_pos.start(),
                    *end_pos.end()
                ),
                path: name_tok.value().to_string(),
            });
        }

        None
    }
}

impl Include {
    pub fn position(&self) -> TokenPosition {
        self.position
    }
    pub fn path(&self) -> &str {
        &self.path
    }
}