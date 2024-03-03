use std::process::exit;
use crash_ir_lexer::{Token, TokenPosition, TokenType};
use crate::stream::TokenStream;

impl TokenStream {
    
    
    pub fn err_unexpected_token(tok: Token) -> ! {
        println!("Unexpected `{}` at {}", tok.clone().value(), format_pos(tok.pos()));
        
        exit(1)
    }
    
    pub fn err_expected_other_token(expected_type: TokenType, actual_tok: Token) -> ! {
        println!("Expected {:?} but found `{}` at {}", expected_type, actual_tok.clone().value(), format_pos(actual_tok.pos()));
        
        exit(1)
    }
    
    pub fn err_text(tok: Token, msg: &str) -> ! {
        println!("{} at {}", msg, format_pos(tok.pos()));
        
        exit(1)
    }
}

fn format_pos(tok_pos: TokenPosition) -> String {
    let start=  tok_pos.start();
    let end = tok_pos.end();
    format!(
        "[{}:{}-{}:{}]", 
        start.line(), 
        start.column(),
        end.line(),
        end.column()
    )
}