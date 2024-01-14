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

pub enum StatementDecl {
    If { expr: ExpressionDecl, code: Vec<StatementDecl> },
    Switch { expr: ExpressionDecl, bodies: Vec<SwitchBodyDecl> },
    Match { expr: ExpressionDecl, bodies: Vec<MatchBodyDecl> },
    Loop { code: Vec<StatementDecl> },
    Return { expr: Option<ExpressionDecl> },
    Break,
    Continue,
    Throw { expr: ExpressionDecl },
    Variable { name: String, typ: Type, mutable: bool, expr: ExpressionDecl },
    Expression { expr: ExpressionDecl },
}

pub struct SwitchBodyDecl {
    target: String,
    code: Vec<StatementDecl>,
}

pub struct MatchBodyDecl {
    target: String,
    params: Vec<String>,
    code: Vec<StatementDecl>,
}