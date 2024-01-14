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

use crate::decl::expression::ExpressionDecl;
use crate::decl::types::Type;

pub struct ConstantDecl {
    name: String,
    typ: Type,
    expr: ExpressionDecl,
}

pub struct FuncParameter {
    typ: Type,
    name: String,
    mutable: bool,
}

pub struct Generic {

}

impl ConstantDecl {
    pub fn new(name: String, typ: Type, expr: ExpressionDecl) -> Self {
        Self { name, typ, expr }
    }
}

impl FuncParameter {
    pub fn new(typ: Type, name: String, mutable: bool) -> Self {
        Self { typ, name, mutable }
    }
}

