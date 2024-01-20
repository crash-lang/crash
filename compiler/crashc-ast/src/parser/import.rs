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

use crashc_lexer::token::TokenKind::*;
use crate::{current_tok, next_tok};
use crate::parser::Cursor;

// Returns option, because we want to use macros
pub fn parse_imports(mut cursor: Cursor) -> Option<Vec<String>> {
    let mut imports = Vec::new();

    loop {
        if let Some(tok) = cursor.current_tok() {
            let kind = tok.kind();

            if kind == Eof {
                break;
            }

            if kind == Import {
                cursor.bump();

                let mut current = current_tok!(cursor);
                let mut current_kind = current.kind();

                // We have a multi-import
                if current_kind == OpenCurlyBrace {
                    // skip {
                    cursor.bump();

                    loop {
                        current = current_tok!(cursor);
                        current_kind = current.kind();

                        if current_kind == CloseCurlyBrace {
                            // Skip }
                            cursor.bump();
                            break
                        }

                        if current_kind != Identifier {
                            panic!("Not identifier with comma")
                        }

                        imports.push(current.content().to_string());

                        let next_kind = next_tok!(cursor).kind();

                        if next_kind != Comma || next_kind != CloseCurlyBrace {
                            panic!("Invalid multi-import syntax")
                        }

                        // Skip identifier and ,
                        cursor.bump();
                        cursor.bump();
                    }

                    continue
                }

                // We have a single import
                if current_kind == Identifier {
                    if next_tok!(cursor).kind() != Semicolon {
                        panic!("Single import not closed with Semicolon")
                    }

                    imports.push(current.content().to_string());

                    // Skip identifier and ;
                    cursor.bump();
                    cursor.bump();
                    continue
                }

                panic!("Something's weird in imports")
            }
        }
    }

    Some(imports)
}