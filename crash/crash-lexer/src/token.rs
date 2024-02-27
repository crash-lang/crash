use crate::position::{INVALID_TOKEN_POSITION, TokenPosition};

const EOF_TOKEN: Token = Token {
    tok_type: TokenType { id: -1, name: "", skipped: false },
    value: String::new(),
    position: INVALID_TOKEN_POSITION
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'a> {
    tok_type: TokenType<'a>,
    value: String,
    position: TokenPosition
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TokenType<'a> {
    id: i16,
    name: &'a str,
    skipped: bool
}

impl<'a> Token<'a> {

    pub fn new(tok_type: TokenType<'a>, value: String, position: TokenPosition) -> Self {
        Self { tok_type, value, position }
    }

    pub fn eof() -> Token<'static> {
        EOF_TOKEN
    }

    pub fn tok_type(&self) -> &TokenType {
        &self.tok_type
    }
    pub fn value(&self) -> &str {
        &self.value
    }
    pub fn position(&self) -> TokenPosition {
        self.position
    }
}

impl<'a> TokenType<'a> {
    pub fn new(id: i16, name: &'a str, skipped: bool) -> Self {
        Self { id, name, skipped }
    }

    pub fn id(&self) -> i16 {
        self.id
    }

    pub fn name(&self) -> &'a str {
        self.name
    }

    pub fn skipped(&self) -> bool {
        self.skipped
    }
}