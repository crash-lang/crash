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

use crate::statements::StatementType;

pub mod expressions;
pub mod statements;
pub mod types;
pub mod misc;

pub struct Module {
    name: String,
    package_name: String,
    content: String,
    statements: Vec<StatementType>
}

pub struct Root {
    modules: Vec<Module>
}

impl Module {
    pub fn new(name: String, package_name: String, content: String, statements: Vec<StatementType>) -> Self {
        Self { name, package_name, content, statements }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn package_name(&self) -> &str {
        &self.package_name
    }
    pub fn statements(&self) -> &Vec<StatementType> {
        &self.statements
    }
}

impl Root {
    pub fn new(modules: Vec<Module>) -> Self {
        Self { modules }
    }
    pub fn modules(&self) -> &Vec<Module> {
        &self.modules
    }
}