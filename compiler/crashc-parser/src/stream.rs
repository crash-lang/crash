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

use crashc_lexer::token::Token;
use crashc_lexer::token::TokenType::Eof;

pub struct TokenStream {
    module_name: String,
    original: Vec<Token>,
    source: String,
    tokens: Vec<Token>,
    index: usize
}

impl TokenStream {
    pub fn new(module_name: String, source: String, tokens: Vec<Token>) -> Self {
        Self { module_name, original: tokens.clone() , source, tokens, index: 0 }
    }

    pub(crate) fn has_more_tokens(&self) -> bool {
        if self.tokens.len() <= self.index {
            return false;
        }

        match self.tokens.get(self.index) {
            None => false,
            Some(tok) => tok.typ() != Eof
        }
    }

    fn tok_at_index(&self, index: usize) -> Token {
        match self.tokens.get(index) {
            None => Token::eof(),
            Some(token) => token.clone()
        }
    }

    pub(crate) fn current(&self) -> Token {
        self.tok_at_index(self.index)
    }

    pub(crate) fn prev(&self) -> Token {
        self.peak(1)
    }

    pub(crate) fn peak(&self, amount: usize) -> Token {
        self.tok_at_index(self.index + amount)
    }

    pub(crate) fn advance(&mut self) {
        self.add(1)
    }

    pub(crate) fn reverse(&mut self) {
        self.remove(1)
    }

    pub(crate) fn add(&mut self, amount: usize) {
        self.index += amount
    }

    pub(crate) fn remove(&mut self, amount: usize) {
        self.index -= amount
    }

    pub fn tokens(&self) -> &Vec<Token> {
        &self.original
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn module_name(self) -> String {
        self.module_name
    }
}