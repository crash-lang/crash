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
use crashc_lexer::token::Token;

#[derive(Clone)]
pub struct Cursor<'a> {
    len_remaining: usize,
    tokens: Iter<'a, Token>
}

impl<'a> Cursor<'a> {

    pub(crate) fn new(tokens: Iter<'a, Token>) -> Cursor<'a> {
        Cursor {
            len_remaining: tokens.len(),
            tokens
        }
    }

    pub(crate) fn current_tok(&self) -> Option<&Token> {
        self.tokens.clone().next()
    }

    pub(crate) fn advance(&mut self) {
        if let Some(_tok) = self.tokens.next() {
            self.len_remaining -= 1;
        }
    }

    pub(crate) fn next(&self) -> Option<&Token> {
        let mut iterator = self.tokens.clone();
        iterator.next();
        iterator.next()
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.tokens.len() == 0
    }

    pub(crate) fn pos_within_token(&self) -> u32 {
        (self.len_remaining - self.tokens.len()) as u32
    }

    pub(crate) fn bump(&mut self) -> Option<&Token> {
        let tok = self.tokens.next()?;
        Some(tok)
    }

    pub(crate) fn len_remaining(&self) -> usize {
        self.len_remaining
    }
}