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
use crate::expressions::literal::Literal;

#[derive(Clone, Debug)]
pub struct Assign {
    position: TokenPosition,
    target: Literal,
    value: Box<ExpressionType>
}

impl Assign {
    pub fn new(position: TokenPosition, target: Literal, value: ExpressionType) -> Self {
        Self { position, target, value: Box::new(value) }
    }

    pub fn target(&self) -> &Literal {
        &self.target
    }
    pub fn value(&self) -> &ExpressionType {
        &self.value
    }
}

impl Expression for Assign {
    fn expression_type(self) -> ExpressionType {
        ExpressionType::Assign(self)
    }

    fn position(&self) -> TokenPosition {
        self.position
    }
}