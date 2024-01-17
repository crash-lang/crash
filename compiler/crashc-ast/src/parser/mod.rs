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

use std::slice::Iter;
use crashc_lexer::token::{Token, TokenKind};

pub(crate) mod import;
pub(crate) mod class;
pub(crate) mod functions;
pub(crate) mod constants;
pub(crate) mod enumeration;
mod macros;

pub struct Cursor<'a> {
    len_remaining: usize,
    tokens: Iter<'a, Token>,
    prev: &'a Token
}

const EOF_TOKEN: &Token = &Token::new(TokenKind::Eof, 0, "");

impl<'a> Cursor<'a> {

    pub(crate) fn new(tokens: Iter<Token>) -> Cursor<'a> {
        Cursor {
            len_remaining: tokens.len(),
            tokens: tokens,
            prev: EOF_TOKEN
        }
    }

    pub(crate) fn current_tok(&self) -> Option<&Token> {
        self.tokens.clone().next()
    }

    pub(crate) fn advance(&mut self) {
        if let Some(tok) = self.tokens.next() {
            self.prev = tok;
            self.len_remaining -= 1;
        }
    }

    pub(crate) fn next(&self) -> &Token {
        let mut iterator = self.tokens.clone();
        iterator.next();
        iterator.next().unwrap_or(EOF_TOKEN)
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.tokens.len() == 0
    }

    pub(crate) fn pos_within_token(&self) -> u32 {
        (self.len_remaining - self.tokens.len()) as u32
    }

    pub(crate) fn bump(&mut self) -> Option<&Token> {
        let tok = self.tokens.next()?;
        self.prev = tok;
        Some(tok)
    }

    pub(crate) fn len_remaining(&self) -> usize {
        self.len_remaining
    }

    pub(crate) fn prev(&self) -> &'a Token {
        self.prev
    }
}