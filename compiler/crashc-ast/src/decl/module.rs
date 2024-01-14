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

use crate::decl::class::ClassDecl;
use crate::decl::enumeration::EnumDecl;
use crate::decl::interface::InterfaceDecl;
use crate::decl::misc::{ConstantDecl, FuncParameter};
use crate::decl::modifier::AccessModifierDecl;
use crate::decl::statement::StatementDecl;
use crate::decl::types::Type;

pub struct ModuleDecl {
    name: String,
    imports: Vec<String>,
    constants: Vec<ConstantDecl>,
    functions: Vec<FunctionDecl>,
    class: Option<ClassDecl>,
    enumeration: Option<EnumDecl>,
    interface: Option<InterfaceDecl>,
}

pub struct FunctionDecl {
    access: AccessModifierDecl,
    name: String,
    ret: Type,
    param: Vec<FuncParameter>,
    code: Vec<StatementDecl>,
}

impl ModuleDecl {
    pub fn new(name: String, imports: Vec<String>, constants: Vec<ConstantDecl>, functions: Vec<FunctionDecl>, class: Option<ClassDecl>, enumeration: Option<EnumDecl>, interface: Option<InterfaceDecl>) -> Self {
        Self { name, imports, constants, functions, class, enumeration, interface }
    }
}

impl FunctionDecl {
    pub fn new(access: AccessModifierDecl, name: String, ret: Type,
               param: Vec<FuncParameter>, code: Vec<StatementDecl>) -> Self {
        Self { access, name, ret, param, code }
    }
}