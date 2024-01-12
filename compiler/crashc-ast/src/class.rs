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

use crate::expression::Expression;
use crate::general::{FuncParameter, Generic};
use crate::modifier::AccessModifier;
use crate::statement::Statement;
use crate::types::Type;

pub struct Class {
    fields: Vec<Field>,
    constructors: Vec<Constructor>,
    methods: Vec<Method>,
    generics: Vec<Generic>,
}

pub struct Field {
    name: String,
    typ: Type,
    mutable: bool,
    expr: Option<Expression>,
}

pub struct Constructor {
    access: AccessModifier,
    /// If the constructor is abstract or not
    abst: bool,
    param: Vec<FuncParameter>,
    code: Option<Vec<Statement>>,
}

/// A method works like a function, but is always inside a class
pub struct Method {
    access: AccessModifier,
    name: String,
    /// If the method is abstract or not
    abst: bool,
    /// Return type
    ret: Type,
    param: Vec<FuncParameter>,
    code: Option<Vec<Statement>>,
}

impl Method {
    pub fn new(access: AccessModifier, name: String, abst: bool, ret: Type, param: Vec<FuncParameter>, code: Option<Vec<Statement>>) -> Self {
        Method { access, name, abst, ret, param, code }
    }
}