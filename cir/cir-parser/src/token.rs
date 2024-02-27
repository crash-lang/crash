use crash_lexer::{Token, TokenPosition, TokenType};
use TokenTypes::*;

pub enum TokenTypes {
    Whitespace,

    // Instructions
    Move,
    Init,
    Push,
    Pop,
    Call,
    Ret,

    // Keywords
    Struct,
    Fn,

    // Symbols
    OpenCurlyBrace,
    CloseCurlyBrace,
    OpenBrace,
    CloseBrace,
    OpenSquareBrace,
    CloseSquareBrace,
    Semicolon,
    Comma,
    
    // Literals
    BooleanLiteral,
    StringLiteral,
    CharLiteral,
    IntegerLiteral,
    FloatLiteral
}

impl TokenTypes {

    pub fn create_tok(&self, value: String, position: TokenPosition) -> Token {
        Token::new(self.as_type(), value, position)
    }

    pub fn as_type(&self) -> TokenType {
        match self {
            Whitespace => TokenType::new(0, "", true),

            Move => TokenType::new(100, "mov", false),
            Init => TokenType::new(101, "init", false),
            Push => TokenType::new(102, "push", false),
            Pop => TokenType::new(103, "pop", false),
            Call => TokenType::new(104, "call", false),
            Ret => TokenType::new(105, "ret", false),

            Struct => TokenType::new(2, "struct", false),
            Fn => TokenType::new(1, "fn", false),

            OpenCurlyBrace => TokenType::new(3, "{", false),
            CloseCurlyBrace => TokenType::new(4, "}", false),
            OpenBrace => TokenType::new(5, "(", false),
            CloseBrace => TokenType::new(6, ")", false),
            OpenSquareBrace => TokenType::new(7, "[", false),
            CloseSquareBrace => TokenType::new(8, "]", false),
            Semicolon => TokenType::new(9, ";", false),
            Comma => TokenType::new(10, ",", false),
            
            BooleanLiteral => TokenType::new(200, "true/false", false),
            StringLiteral => TokenType::new(201, "\"\"", false),
            CharLiteral => TokenType::new(202, "\'\'", false),
            IntegerLiteral => TokenType::new(203, "", false),
            FloatLiteral => TokenType::new(204, "", false)
        }
    }
}