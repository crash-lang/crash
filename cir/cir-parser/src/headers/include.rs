use std::path::Path;
use crash_ir_lexer::TokenType;
use crate::stream::TokenStream;
use crate::tok_struct;

tok_struct!(
    Include <'a> {
        path: &'a Path
    }
);

impl TokenStream {

    pub fn try_parse_include(&mut self) -> Option<Include> {
        match self.try_token(TokenType::Include) {
            Some(tok) => {
                let mut tokens = vec![tok];
                let start_pos = tok.pos().start();

                let path_literal = self.expect_path();
                tokens.append(&mut path_literal.tokens());


                let semicolon_tok = self.expect_token(TokenType::Semicolon);
                tokens.push(semicolon_tok.clone());

                let end_pos = semicolon_tok.pos().end();

                return Some(Include {
                    tokens,
                    position: TokenPosition::new(
                        start_pos,
                        end_pos
                    ),
                    path: path_literal.path(),
                });
            },
            None => None
        }
    }
}

impl<'a> Include<'a> {
    pub fn path(&self) -> &'a Path {
        self.path
    }
}