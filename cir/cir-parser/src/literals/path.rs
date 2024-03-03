use std::path::Path;
use crash_ir_lexer::{TokenType};
use crate::stream::TokenStream;
use crate::tok_struct;

tok_struct!(
    PathLiteral<'a> {
        path: &'a Path
    }
);

impl TokenStream {

    pub fn expect_path(&mut self) -> PathLiteral {
        let tok = self.expect_token(TokenType::PathLiteral);
        let path = Path::new(tok.value());

        PathLiteral {
            tokens: vec![tok],
            position: tok.pos(),
            path,
        }
    }

    pub fn try_parse_path(&mut self) -> Option<PathLiteral> {
        match self.try_token(TokenType::PathLiteral) {
            None => None,
            Some(tok) => {
                let path = Path::new(tok.value());

                Some(PathLiteral {
                    tokens: vec![tok],
                    position: tok.pos(),
                    path,
                })
            }
        }
    }
}

impl<'a> PathLiteral<'a> {
    pub fn path(&self) -> &'a Path {
        self.path
    }
}