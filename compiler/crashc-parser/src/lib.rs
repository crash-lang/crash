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

mod statement;
mod stream;
mod modifier;
mod import;
mod macros;
mod function;
mod constructor;

use crashc_tree::{Module};
use crate::statement::parse_statement;
use crate::stream::{build_stream};

pub async fn parse_module(module_name: String, package_name: String, content: String) -> Module {
    let mut statements = Vec::new();

    let mut stream = build_stream(content.clone(), module_name.clone());

    while stream.has_more_tokens() {
        if let Some(statement) = parse_statement(&mut stream) {
            statements.push(statement);
        }
    }

    Module::new(module_name, package_name, content, statements)
}