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
use crate::current_tok;
use crate::decl::class::{ClassDecl, ConstructorDecl, FieldDecl, MethodDecl};
use crate::decl::misc::Generic;
use crate::parser::Cursor;

pub fn parse_class(mut cursor: Cursor) -> Option<ClassDecl> {

    loop {
        if let Some(tok) = cursor.current_tok() {
            let kind = tok.kind();

            if kind == Eof {
                break;
            }

            if kind == Class {
                let mut fields: Vec<FieldDecl> = Vec::new();
                let mut constructors: Vec<ConstructorDecl> = Vec::new();
                let mut methods: Vec<MethodDecl> = Vec::new();
                let mut generics: Vec<Generic> = Vec::new();

                cursor.bump();

                let mut current = cursor.next();
                let mut current_kind = current.kind();

                cursor.bump();

                let class_name;

                // Optional class name
                if current_kind == Identifier {
                    class_name = Some(current.content());
                    cursor.bump();
                    current = current_tok!(cursor);
                    current_kind = current.kind();
                }

                // Optional Generics
                if current_kind == Less {
                    cursor.bump();
                    loop {
                        current = current_tok!(cursor);
                        current_kind = current.kind();

                        let next = cursor.next();
                        let next_kind = next.kind();

                        if current_kind == Greater {
                            cursor.bump();
                            current = current_tok!(cursor);
                            current_kind = current.kind();
                            break;
                        }

                        if current_kind == Identifier {
                            generics.push(Generic::new(current.content().to_string()));

                            if next_kind == Comma {
                                cursor.bump();
                            }

                            cursor.bump();
                            continue;
                        }
                        panic!("Invalid symbol in generics")
                    }
                }

                if current_kind != OpenCurlyBrace {
                    panic!("Invalid class syntax")
                }

                // Skip {
                cursor.bump();
                current = current_tok!(cursor);
                current_kind = current.kind();



            }
        }
        break;
    }

    None
}