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

pub use crate::stream::TokenStream;
use crate::position::StructurePosition;
use crate::statement::Statement;

pub mod expr;
pub mod stream;
pub mod statement;
pub mod structures;
pub mod position;

struct Module {
    module_name: String,
    statements: Vec<Statement>
}

impl TokenStream {

    pub fn parse_module(&mut self) -> Module {
        let mut module = Module { module_name: self.module_name(), statements: Vec::new() };

        loop {
            if !self.has_more_tokens() {
                break;
            }

            match self.try_parse_statement() {
                None => {
                    //TODO syntax handling
                }
                Some(statement) => {
                    module.statements.push(statement);
                }
            }
        }

        module
    }
}

pub trait Structure {

    fn pos(&self) -> StructurePosition;
}