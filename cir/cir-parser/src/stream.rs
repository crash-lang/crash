use std::sync::{Arc, Mutex};
use crash_ir_lexer::{Token, tokenize, TokenType};

pub(crate) struct TokenStream {
    tokens: Vec<Token>,
    index: usize,
    content: String
}

impl TokenStream {

    pub fn new(content: String) -> Self {
        Self {
            tokens: tokenize(&content),
            index: 0,
            content
        }
    }
    
    pub fn expect_token(mutex_stream: Arc<Mutex<Self>>, token_type: TokenType) -> Token {
        let mut stream = mutex_stream.lock().unwrap();
        let mut tok = stream.current();
        
        if tok.tok_type() != token_type {
            Self::err_expected_other_token(token_type, tok)
        }
        
        stream.advance();
        
        tok
    }
    
    /// If the given token-type matches with the current token,
    /// the index is advanced and the token returned.
    /// Otherwise, nothing happens.
    pub fn try_token(mutex_stream: Arc<Mutex<Self>>, token_type: TokenType) -> Option<Token> {
        let mut stream = mutex_stream.lock().unwrap();
        
        let mut tok = stream.current();
        
        if tok.tok_type() != token_type {
            return None;
        }
        
        stream.advance();
        
        return Some(tok);
    }

    pub fn tok_at_index(&self, index: usize) -> Token {
        match self.tokens.get(index) {
            None => Token::eof(),
            Some(tok) => tok.clone()
        }
    }

    pub fn current(&self) -> Token {
        self.tok_at_index(self.index)
    }

    pub fn prev(&self) -> Token {
        self.tok_at_index(self.index - 1)
    }

    pub fn peak(&self, amount: usize) -> Token {
        self.tok_at_index(self.index + amount)
    }

    pub fn advance(&mut self) {
        self.add(1)
    }

    pub fn reverse(&mut self) {
        self.remove(1)
    }

    pub fn add(&mut self, amount: usize) {
        self.index += amount
    }

    pub fn remove(&mut self, amount: usize) {
        self.index -= amount
    }

    pub fn has_more_tokens(&self) -> bool {
        if self.tokens.len() <= self.index {
            return false;
        }

        match self.tokens.get(self.index) {
            None => false,
            Some(tok) => tok.tok_type() != TokenType::Eof
        }
    }
    
    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}