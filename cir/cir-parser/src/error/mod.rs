use std::process::exit;
use crash_ir_lexer::{Token, TokenType};

pub fn expected_token_error(token_type: TokenType, token: Token) -> ! {
    println!("Expected token-type {:?} but found {:?}", token_type, token);    
    exit(1)
}