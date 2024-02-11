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

use crashc_lexer::token::TokenType::{CloseCurlyBrace, OpenCurlyBrace};
use crate::expr::Expr;
use crate::position::StructurePosition;
use crate::statement::Statement;
use crate::stream::TokenStream;
use crate::Structure;

pub struct StatementBody {
    position: StructurePosition,
    statements: Vec<Statement>
}

impl TokenStream {

    pub(crate) fn try_parse_statement_body(&mut self) -> Option<StatementBody> {
        let current_tok = self.current();
        let current_type = current_tok.typ();

        if current_type == OpenCurlyBrace {
            let start_pos = current_tok.position();

            self.advance();

            let mut statements = Vec::new();

            loop {
                if self.current().typ() == CloseCurlyBrace {
                    let end_pos = current_tok.position();

                    self.advance();

                    return Some(StatementBody { position: StructurePosition::new(
                        start_pos,
                        end_pos
                    ), statements });
                }
                if !self.has_more_tokens() {
                    break;
                }
                if let Some(statement) = self.try_parse_statement() {
                    statements.push(statement);
                    continue;
                }
                break;
            }
        }

        None
    }
}

impl StatementBody {
    pub fn statements(&self) -> &Vec<Statement> {
        &self.statements
    }
}


impl Structure for StatementBody {
    fn pos(&self) -> StructurePosition {
        self.position
    }
}
