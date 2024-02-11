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

mod ifs;
mod imports;
mod matches;

use crate::statement::ifs::IfStatement;
use crate::statement::imports::ImportStatement;
use crate::stream::TokenStream;

pub enum Statement {
    Import(ImportStatement),

    If(IfStatement)
}

impl TokenStream {

    pub(super) fn try_parse_statement(&mut self) -> Option<Statement> {
        if let Some(import_statement) = self.try_parse_import_statement() {
            return Some(Statement::Import(import_statement));
        }

        if let Some(if_statement) = self.try_parse_if_statement() {
            return Some(Statement::If(if_statement));
        }

        None
    }
}