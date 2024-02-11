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
use crate::expr::Expr;
use crate::position::StructurePosition;
use crate::stream::TokenStream;
use crate::Structure;
use crate::structures::StatementBody;

pub struct IfStatement {
    position: StructurePosition,
    expr: Expr,
    body: StatementBody
}

impl TokenStream {

    pub(super) fn try_parse_if_statement(&mut self) -> Option<IfStatement> {
        let current_tok = self.current();
        let current_type = current_tok.typ();

        if current_type == TokenType::If {
            let start_pos = current_tok.position();

            self.advance();

            let expr = match self.try_parse_expr() {
                None => {
                    //TODO syntax-error handling
                    println!("Invalid if syntax");
                    return None;
                }
                Some(expr) => expr
            };

            let body = match self.try_parse_statement_body() {
                None => {
                    //TODO syntax-error handling
                    println!("No body in if");
                    return None;
                }
                Some(body) => body
            };

            return Some(IfStatement { position: StructurePosition::new(
                start_pos,
                body.pos().end()
            ), expr, body });
        }

        None
    }
}

impl IfStatement {
    pub fn expr(&self) -> &Expr {
        &self.expr
    }
    pub fn body(&self) -> &StatementBody {
        &self.body
    }
}

impl Structure for IfStatement {
    fn pos(&self) -> StructurePosition {
        self.position
    }
}