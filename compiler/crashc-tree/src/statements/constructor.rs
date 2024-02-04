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
use crate::statements::function::FunctionParameter;
use crate::statements::{StatementType, Statement};

#[derive(Clone, Debug)]
pub struct Constructor {
    position: TokenPosition,
    modifiers: Vec<Modifier>,
    parameters: Vec<FunctionParameter>,
}

define_statement!(Constructor);