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
 */

use std::str::Chars;

pub struct Cursor<'a> {
    len_remaining: usize,
    chars: Chars<'a>,
    prev: char
}

pub(crate) const EOF_CHAR: char = '\0';

pub(crate) fn is_whitespace(c: char) -> bool {
    matches!(c, '\u{0009}' | '\u{000A}'
        | '\u{000B}' | '\u{000C}'
        | '\u{000D}' | '\u{0020}'
        | '\u{0085}' | '\u{200E}'
        | '\u{200F}' | '\u{2028}'
        | '\u{2029}'
    )
}

impl<'a> Cursor<'a> {

    pub fn new(input: &'a str) -> Cursor<'a> {
        Cursor {
            len_remaining: input.len(),
            chars: input.chars(),
            prev : EOF_CHAR
        }
    }

    pub fn as_str(&self) -> &'a str {
        self.chars.as_str()
    }

    pub(crate) fn current_char(&self) -> Option<char> {
        self.chars.clone().next()
    }

    pub(crate) fn advance(&mut self) {
        if let Some(c) = self.chars.next() {
            self.prev = c;
            self.len_remaining -= 1;
        }
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if !is_whitespace(c) {
                break
            }
            self.advance()
        }
    }

    pub(crate) fn next(&self) -> char {
        let mut iterator = self.chars.clone();
        iterator.next();
        iterator.next().unwrap_or(EOF_CHAR)
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    pub(crate) fn pos_within_token(&self) -> u32 {
        (self.len_remaining - self.chars.as_str().len()) as u32
    }

    pub(crate) fn bump(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.prev  = c;
        Some(c)
    }

    pub fn chars(&self) -> &Chars<'a> {
        &self.chars
    }

    pub fn len_remaining(&self) -> usize {
        self.len_remaining
    }
}