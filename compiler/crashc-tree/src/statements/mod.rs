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

pub mod import;
pub mod modifier_block;
pub mod function;
pub mod constructor;

use crashc_lexer::position::TokenPosition;
use crate::statements::constructor::Constructor;
use crate::statements::function::Function;
use crate::statements::import::Import;
use crate::statements::modifier_block::ModifierBlock;

#[derive(Clone, Debug)]
pub enum StatementType {
    Import(Import),
    ModifierBlock(ModifierBlock),
    Function(Function),
    Constructor(Constructor),
}

pub trait Statement {

    fn statement_type(self) -> StatementType;

    fn position(&self) -> TokenPosition;
}

#[macro_export]
macro_rules! define_statement {
    ($name:ident) => {
        impl Statement for $name {
            fn statement_type(self) -> StatementType {
                StatementType::$name(self)
            }

            fn position(&self) -> TokenPosition {
                self.position
            }
        }
    };
}