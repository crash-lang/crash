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

use crashc_lexer::token::TokenType;
use crashc_lexer::token::TokenType::{CloseCurlyBrace, OpenCurlyBrace};
use crate::expr::Expr;
use crate::position::StructurePosition;
use crate::stream::TokenStream;
use crate::Structure;

pub struct ImportStatement {
    position: StructurePosition,
    expressions: Vec<Expr>
}

impl TokenStream {

    pub(super) fn try_parse_import_statement(&mut self) -> Option<ImportStatement> {
        let mut current_tok = self.current();
        let mut current_type = current_tok.typ();

        if current_type == TokenType::Import {
            let start_pos = current_tok.position();

            self.advance();

            if let Some(expr) = self.try_parse_expr() {
                return Some(ImportStatement {
                    position: StructurePosition::new(
                        start_pos,
                        // Since we used try_parse_expr, the expr is already advanced
                        expr.pos().end()
                    ),
                    expressions: vec![expr]
                })
            }

            current_tok = self.current();
            current_type = current_tok.typ();

            if current_type == OpenCurlyBrace {
                self.advance();

                let mut expressions = Vec::new();

                loop {
                    current_tok = self.current();
                    current_type = current_tok.typ();

                    if current_type == CloseCurlyBrace {
                        let end_pos = current_tok.position();

                        self.advance();

                        return Some(ImportStatement {
                            position: StructurePosition::new(start_pos, end_pos),
                            expressions
                        })
                    }
                    if !self.has_more_tokens() {
                        break;
                    }
                    if let Some(expr) = self.try_parse_expr() {
                        expressions.push(expr);
                        continue;
                    }
                    break;
                }
            }

            //TODO Syntax-errors
            println!("Wrong syntax");
        }

        None
    }
}

impl ImportStatement {
    pub fn expressions(&self) -> &Vec<Expr> {
        &self.expressions
    }
}

impl Structure for ImportStatement {
    fn pos(&self) -> StructurePosition {
        self.position
    }
}