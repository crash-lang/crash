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

pub use crate::rule::LexingRule;
use crate::token::{Position, Token};
use crate::token::TokenType::{Eof, Whitespace};

pub struct Lexer {
    rules: Vec<LexingRule>
}

impl Lexer {

    pub fn new(rules: Vec<LexingRule>) -> Lexer {
        Self { rules }
    }

    pub fn tokenize(&mut self, mut input: &str, skip_whitespace: bool) -> Vec<Token> {
        let mut tokens = Vec::new();

        let mut current_column = 1;
        let mut current_line = 1;

        while !input.is_empty() {
            let mut matched_length = 0;
            let mut matched_type = None;

            for rule in self.rules.iter() {
                let pattern = rule.patterns();

                for pattern in pattern {
                    if let Some(matcher) = pattern.find(input) {
                        if matcher.start() == 0 {
                            let length = matcher.end();

                            if length > matched_length {
                                matched_length = length;
                                matched_type = Some(rule.typ());
                            }
                        }
                    }
                }
            }

            if matched_length > 0 {
                let substring = &input[..matched_length];

                let token = Token::new(
                    matched_type.unwrap(),
                    String::from(substring),
                    Position::new(current_line, current_column)
                );

                tokens.push(token);

                for c in substring.chars() {
                    if c == '\n' {
                        current_line += 1;
                        current_column = 1;
                        continue;
                    }
                    current_column += 1;
                }

                input = &input[matched_length..];
                continue;
            }

            println!("Unknown token at {:?}:{:?}", current_line, current_column);
            std::process::exit(1);
        }

        if skip_whitespace {
            tokens.retain(|token| token.typ() != Whitespace);
        }

        tokens.push(Token::new(Eof, String::new(), Position::new(0, 0)));

        tokens
    }
}