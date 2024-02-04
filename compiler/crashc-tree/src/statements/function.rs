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
use crate::misc::Modifier;
use crate::statements::{StatementType, Statement};
use crate::types::Type;

#[derive(Clone, Debug)]
pub struct Function {
    position: TokenPosition,
    modifiers: Vec<Modifier>,

    return_type: Box<Type<'static>>,
    parameters: Vec<FunctionParameter>,
}

#[derive(Clone, Debug)]
pub struct FunctionParameter {
    position: TokenPosition,
    modifiers: Vec<Modifier>,

}

define_statement!(Function);

impl Function {
    pub fn new(position: TokenPosition, modifiers: Vec<Modifier>, return_type: Type<'static>, parameters: Vec<FunctionParameter>) -> Self {
        Self { position, modifiers, return_type: Box::new(return_type), parameters }
    }

    pub fn position(&self) -> TokenPosition {
        self.position
    }
    pub fn modifiers(&self) -> &Vec<Modifier> {
        &self.modifiers
    }
    pub fn return_type(&self) -> &Type<'static> {
        &self.return_type
    }
    pub fn parameters(&self) -> &Vec<FunctionParameter> {
        &self.parameters
    }
}

impl FunctionParameter {
    pub fn new(position: TokenPosition, modifiers: Vec<Modifier>) -> Self {
        Self { position, modifiers }
    }
}