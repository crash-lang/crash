pub use {
    crate::position::Position,
    crate::position::TokenPosition
};

use crate::position::INVALID_TOKEN_POS;

const EOF_TOK: Token = Token {
    tok_type: TokenType::Eof,
    value: String::new(),
    position: INVALID_TOKEN_POS,
};

#[derive(Debug, Clone)]
pub struct Token {
    tok_type: TokenType,
    value: String,
    position: TokenPosition
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenType {
    Eof,
    Whitespace,

    Include,
    Const,
    
    PathLiteral,
    BooleanLiteral,
    StringLiteral,
    CharLiteral,
    IntegerLiteral,
    FloatLiteral,
    
    Semicolon,
}

impl Token {
    pub fn new(tok_type: TokenType, value: String, position: Position) -> Self {
        let length= value.len();
        Self {
            tok_type,
            value,
            position: TokenPosition::new(position, Position::new(
                position.line(),
                position.column() + (length as i32)))
        }
    }

    pub fn eof() -> Self {
        EOF_TOK
    }

    pub fn tok_type(&self) -> TokenType {
        self.tok_type
    }
    
    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn pos(self) -> TokenPosition {
        self.position
    }
}