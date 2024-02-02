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
pub struct Addition{
    position: TokenPosition,
    summand: Vec<ExpressionType>
}

#[derive(Clone, Debug)]
pub struct Subtraction {
    position: TokenPosition,
    subtrahends: Vec<ExpressionType>
}

#[derive(Clone, Debug)]
pub struct Multiplication {
    position: TokenPosition,
    factors: Vec<ExpressionType>
}

#[derive(Clone, Debug)]
pub struct Division {
    position: TokenPosition,
    dividends: Vec<ExpressionType>
}

#[derive(Clone, Debug)]
pub struct Modulus {
    position: TokenPosition,
    components: Vec<ExpressionType>
}

define_expression!(Addition);
define_expression!(Subtraction);
define_expression!(Multiplication);
define_expression!(Division);
define_expression!(Modulus);

impl Addition {
    pub fn new(position: TokenPosition, summand: Vec<ExpressionType>) -> Self {
        Self { position, summand }
    }

    pub fn summand(&self) -> &Vec<ExpressionType> {
        &self.summand
    }
}

impl Subtraction {
    pub fn new(position: TokenPosition, subtrahends: Vec<ExpressionType>) -> Self {
        Self { position, subtrahends }
    }

    pub fn subtrahends(&self) -> &Vec<ExpressionType> {
        &self.subtrahends
    }
}

impl Multiplication {
    pub fn new(position: TokenPosition, factors: Vec<ExpressionType>) -> Self {
        Self { position, factors }
    }

    pub fn factors(&self) -> &Vec<ExpressionType> {
        &self.factors
    }
}

impl Division {
    pub fn new(position: TokenPosition, dividends: Vec<ExpressionType>) -> Self {
        Self { position, dividends }
    }

    pub fn dividends(&self) -> &Vec<ExpressionType> {
        &self.dividends
    }
}

impl Modulus {
    pub fn new(position: TokenPosition, components: Vec<ExpressionType>) -> Self {
        Self { position, components }
    }

    pub fn components(&self) -> &Vec<ExpressionType> {
        &self.components
    }
}
