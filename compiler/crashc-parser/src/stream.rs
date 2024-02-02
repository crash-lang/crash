/*
 * Copyright 2024 Julian Siebert
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

use crashc_lexer::position::TokenPosition;
use crashc_lexer::token::{Token, TokenType};
use crashc_lexer::token::TokenType::*;
use crashc_lexer::tokenize;

pub fn build_stream(content: String, module_name: String) -> TokenStream {
    let tokens = tokenize(content.as_str());

    TokenStream {
        module_name,
        source: content,
        tokens,
        index: 0,
    }
}

#[derive(Clone)]
pub struct TokenStream {
    module_name: String,
    source: String,
    tokens: Vec<Token>,
    index: usize,
}

impl TokenStream {
    pub fn has_more_tokens(&self) -> bool {
        if self.tokens.len() <= self.index {
            return false;
        }

        match self.tokens.get(self.index) {
            None => false,
            Some(tok) => tok.typ() != Eof
        }
    }

    pub fn matches(&self, token_type: TokenType) -> bool {
        self.current().typ() == token_type
    }

    pub fn matches_value(&self, value: &str) -> bool {
        self.current().value().eq(value)
    }

    pub fn current_pos(&self) -> TokenPosition {
        self.current().position()
    }

    fn tok_at_index(&self, index: usize) -> Token {
        match self.tokens.get(index) {
            None => Token::eof(),
            Some(token) => token.clone()
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
        self.index += 1
    }

    pub fn add(&mut self, index: usize) {
        self.index += index
    }

    pub fn reverse(&mut self) {
        self.index -= 1
    }

    pub fn remove(&mut self, index: usize) {
        self.index -= index
    }

    pub fn source(self) -> String {
        self.source
    }

    pub fn module_name(&self) -> &str {
        &self.module_name
    }
    
    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}