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
use crate::define_expression;
use crate::expressions::{Expression, ExpressionType};

#[derive(Clone, Debug)]
pub struct And {
    position: TokenPosition,
    expressions: Vec<ExpressionType>
}

#[derive(Clone, Debug)]
pub struct Or {
    position: TokenPosition,
    expressions: Vec<ExpressionType>
}

#[derive(Clone, Debug)]
pub struct Not {
    position: TokenPosition,
    expression: Box<ExpressionType>
}

#[derive(Clone, Debug)]
pub struct Equals {
    position: TokenPosition,
    left: Box<ExpressionType>,
    right: Box<ExpressionType>
}

#[derive(Clone, Debug)]
pub struct Greater {
    position: TokenPosition,
    left: Box<ExpressionType>,
    right: Box<ExpressionType>
}

#[derive(Clone, Debug)]
pub struct Less {
    position: TokenPosition,
    left: Box<ExpressionType>,
    right: Box<ExpressionType>
}

#[derive(Clone, Debug)]
pub struct GreaterOrEqual {
    position: TokenPosition,
    left: Box<ExpressionType>,
    right: Box<ExpressionType>
}

#[derive(Clone, Debug)]
pub struct LessOrEqual {
    position: TokenPosition,
    left: Box<ExpressionType>,
    right: Box<ExpressionType>
}

define_expression!(And);
define_expression!(Or);
define_expression!(Not);
define_expression!(Equals);
define_expression!(Greater);
define_expression!(Less);
define_expression!(GreaterOrEqual);
define_expression!(LessOrEqual);

impl And {
    pub fn new(position: TokenPosition, expressions: Vec<ExpressionType>) -> Self {
        Self { position, expressions }
    }

    pub fn expressions(&self) -> &Vec<ExpressionType> {
        &self.expressions
    }
}

impl Or {
    pub fn new(position: TokenPosition, expressions: Vec<ExpressionType>) -> Self {
        Self { position, expressions }
    }

    pub fn expressions(&self) -> &Vec<ExpressionType> {
        &self.expressions
    }
}

impl Not {
    pub fn new(position: TokenPosition, expression: ExpressionType) -> Self {
        Self { position, expression: Box::new(expression) }
    }

    pub fn expression(&self) -> &Box<ExpressionType> {
        &self.expression
    }
}

impl Equals {
    pub fn new(position: TokenPosition, left: ExpressionType, right: ExpressionType) -> Self {
        Self { position, left: Box::new(left), right: Box::new(right) }
    }

    pub fn left(&self) -> &Box<ExpressionType> {
        &self.left
    }

    pub fn right(&self) -> &Box<ExpressionType> {
        &self.right
    }
}

impl Less {
    pub fn new(position: TokenPosition, left: ExpressionType, right: ExpressionType) -> Self {
        Self { position, left: Box::new(left), right: Box::new(right) }
    }

    pub fn left(&self) -> &Box<ExpressionType> {
        &self.left
    }

    pub fn right(&self) -> &Box<ExpressionType> {
        &self.right
    }
}

impl Greater {
    pub fn new(position: TokenPosition, left: ExpressionType, right: ExpressionType) -> Self {
        Self { position, left: Box::new(left), right: Box::new(right) }
    }

    pub fn left(&self) -> &Box<ExpressionType> {
        &self.left
    }

    pub fn right(&self) -> &Box<ExpressionType> {
        &self.right
    }
}

impl LessOrEqual {
    pub fn new(position: TokenPosition, left: ExpressionType, right: ExpressionType) -> Self {
        Self { position, left: Box::new(left), right: Box::new(right) }
    }

    pub fn left(&self) -> &Box<ExpressionType> {
        &self.left
    }

    pub fn right(&self) -> &Box<ExpressionType> {
        &self.right
    }
}

impl GreaterOrEqual {
    pub fn new(position: TokenPosition, left: ExpressionType, right: ExpressionType) -> Self {
        Self { position, left: Box::new(left), right: Box::new(right) }
    }

    pub fn left(&self) -> &Box<ExpressionType> {
        &self.left
    }

    pub fn right(&self) -> &Box<ExpressionType> {
        &self.right
    }
}

