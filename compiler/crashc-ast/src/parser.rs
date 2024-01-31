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
use std::sync::Mutex;
use crashc_lexer::lock_cursor;
use crashc_lexer::token::{Token, TokenKind};
use crashc_lexer::token::TokenKind::{CloseCurlyBrace, Comma, Identifier, OpenCurlyBrace, Semicolon};
use crate::components::{Component, Function, Import, CrashModule};
use crate::cursor::Cursor;

pub struct Parser<'a> {
    tokens: Vec<Token>,
    cursor: Mutex<Cursor<'a>>
}

impl<'a> Parser<'a> {

    pub fn new(tokens: Vec<Token>) -> Parser<'a> {

    }

    pub fn parse_module(&mut self, module_name: String) -> CrashModule {
        let mut cursor = lock_cursor!(self);

        let mut module = CrashModule::new();

        while !cursor.is_eof() {
            self.parse_next(&module);
        }

        module
    }

    fn parse_next(&mut self, mut module: &CrashModule) {
        let mut cursor = lock_cursor!(self);

        if let Some(tok) = cursor.current_tok() {
            drop(cursor);

            let kind = tok.kind();

            if kind == TokenKind::Import {
                if let Some(import) = self.try_parse_import() {
                    module.imports().push(import);
                    return;
                }

                panic!("Invalid import syntax")
            }
        }
    }

    // Idk why this function has to be that long -_-
    fn try_parse_import(&mut self) -> Option<Import> {
        let mut cursor = lock_cursor!(self);

        return match cursor.current_tok() {
            Some(tok) => {
                let kind = tok.kind();

                if kind == TokenKind::Import {
                    let start_index = cursor.pos_within_token();
                    let mut component = Component::new(0, Vec::new());
                    let mut tokens = Vec::new();

                    cursor.bump();

                    let mut index = 0;
                    let mut closed = true;
                    let mut multiple = false;

                    let mut imports = Vec::new();

                    while let Some(current_tok) = cursor.current_tok() {
                        tokens.push(current_tok);

                        cursor.bump();

                        let current_kind = current_tok.kind();

                        // We have a single import
                        if current_kind == Identifier {
                            let next_tok = cursor.next().unwrap();
                            let next_kind = next_tok.kind();

                            if index == 0 {
                                if next_kind != Semicolon {
                                    panic!("Single import not ending with semicolon")
                                }

                                imports.push(current_tok.content().to_string());

                                cursor.bump();
                                cursor.bump();

                                component.set_len((cursor.pos_within_token() - start_index) as usize);
                                component.set_content(tokens);

                                return Some(Import::new(imports, component));
                            }

                            if multiple {
                                if next_kind != Comma || next_kind != CloseCurlyBrace {
                                    panic!("Invalid multi-import syntax")
                                }

                                imports.push(current_tok.content().to_string());

                                cursor.bump();
                                cursor.bump();

                                continue
                            }
                        }

                        // We have a multi-import
                        if current_kind == OpenCurlyBrace && index == 0 {
                            multiple = true;
                            closed = false;
                            continue;
                        }

                        // Random curly-brace on wrong position
                        if current_kind == OpenCurlyBrace {
                            if index == 0 {
                                multiple = true;
                                continue;
                            }
                            if multiple {
                                panic!("Invalid import syntax")
                            }
                        }

                        // Closing multi-import with curly-brace
                        if current_kind == CloseCurlyBrace {
                            if index == 0 || !multiple {
                                panic!("Invalid import syntax")
                            }
                            if closed {
                                panic!("Invalid import syntax")
                            }

                            closed = true;

                            cursor.bump(); // skip curly-brace

                            component.set_len((cursor.pos_within_token() - start_index) as usize);
                            component.set_content(tokens);

                            return Some(Import::new(imports, component));
                        }
                    }
                }
                None
            }
            _ => None
        }
    }

    fn try_parse_function(&mut self) -> Option<Function> {
        let mut cursor = lock_cursor!(self);

        return match cursor.current_tok() {
            Some(tok) => {
                let kind = tok.kind();

                if kind == Ac


                None
            }
            _ => None
        }
    }
}