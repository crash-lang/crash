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
use crate::decl::misc::{FuncParameter, Generic};
use crate::decl::modifier::AccessModifierDecl;
use crate::decl::statement::StatementDecl;
use crate::decl::types::Type;

pub struct ClassDecl {
    name: Option<String>,
    fields: Vec<FieldDecl>,
    constructors: Vec<ConstructorDecl>,
    methods: Vec<MethodDecl>,
    generics: Vec<Generic>,
}

pub struct FieldDecl {
    name: String,
    typ: Type,
    mutable: bool,
    expr: Option<ExpressionDecl>,
}

pub struct ConstructorDecl {
    access: AccessModifierDecl,
    /// If the constructor is abstract or not
    abst: bool,
    param: Vec<FuncParameter>,
    code: Option<Vec<StatementDecl>>,
}

/// A method works like a function, but is always inside a class
pub struct MethodDecl {
    access: AccessModifierDecl,
    name: String,
    /// If the method is abstract or not
    abst: bool,
    /// Return type
    ret: Type,
    param: Vec<FuncParameter>,
    code: Option<Vec<StatementDecl>>,
}

impl MethodDecl {
    pub fn new(access: AccessModifierDecl, name: String, abst: bool, ret: Type, param: Vec<FuncParameter>, code: Option<Vec<StatementDecl>>) -> Self {
        MethodDecl { access, name, abst, ret, param, code }
    }
}

impl ClassDecl {
    pub fn new(name: Option<String>, fields: Vec<FieldDecl>, constructors: Vec<ConstructorDecl>, methods: Vec<MethodDecl>, generics: Vec<Generic>) -> Self {
        Self { name, fields, constructors, methods, generics }
    }
}

impl ConstructorDecl {
    pub fn new(access: AccessModifierDecl, abst: bool, param: Vec<FuncParameter>, code: Option<Vec<StatementDecl>>) -> Self {
        Self { access, abst, param, code }
    }
}

impl FieldDecl {
    pub fn new(name: String, typ: Type, mutable: bool, expr: Option<ExpressionDecl>) -> Self {
        Self { name, typ, mutable, expr }
    }
}