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

use crate::expr::Expr::Literal;
use crate::expr::literal::LiteralExpr;
use crate::position::StructurePosition;
use crate::stream::TokenStream;
use crate::Structure;

mod arithmetic;
mod literal;

pub enum Expr {
    Literal(StructurePosition, LiteralExpr)
}

impl TokenStream {

    pub(crate) fn try_parse_expr(&mut self) -> Option<Expr> {
        let start_pos = self.current().position();

        if let Some(literal_expr) = self.try_parse_literal_expr() {
            return Some(Literal(StructurePosition::new(start_pos, literal_expr.pos().end()), literal_expr));
        }

        None
    }
}

impl Structure for Expr {
    fn pos(&self) -> StructurePosition {
        *match self {
            Literal(pos, _) => pos
        }
    }
}