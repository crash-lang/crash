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

use crashc_lexer::position::TokenPosition;
use crate::expressions::{Expression, ExpressionType};

#[derive(Clone, Debug)]
pub enum Literal {
    Identifier { position: TokenPosition, name: String },
    Integer { position: TokenPosition, number: i128 },
    UnsignedInteger { position: TokenPosition, number: u128 },
    Float { position: TokenPosition, number: f64 },
    Char { position: TokenPosition, char: char },
    Boolean { position: TokenPosition, value: bool },
    String { position: TokenPosition, content: String }
}

impl Expression for Literal {

    fn expression_type(self) -> ExpressionType {
        ExpressionType::Literal(self)
    }

    fn position(&self) -> TokenPosition {
        return *match self {
            Literal::Identifier { position, .. } => position,
            Literal::Integer { position, ..} => position,
            Literal::UnsignedInteger { position, ..} => position,
            Literal::Float { position, ..} => position,
            Literal::Char { position, ..} => position,
            Literal::Boolean { position, ..} => position,
            Literal::String { position, ..} => position,
        }
    }
}