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
use crate::define_statement;
use crate::expressions::ExpressionType;
use crate::statements::{StatementType, Statement};

#[derive(Clone, Debug)]
pub struct If {
    position: TokenPosition,
    expression: ExpressionType,
    statements: Vec<StatementType>
}

define_statement!(If);

impl If {
    pub fn new(position: TokenPosition, expression: ExpressionType, statements: Vec<StatementType>) -> Self {
        Self { position, expression, statements }
    }

    pub fn expression(&self) -> &ExpressionType {
        &self.expression
    }
    pub fn statements(&self) -> &Vec<StatementType> {
        &self.statements
    }
}
