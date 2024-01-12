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
use crate::types::Type;

pub enum Statement {
    If { expr: Expression, code: Vec<Statement> },
    Switch { expr: Expression, bodies: Vec<SwitchBody> },
    Match { expr: Expression, bodies: Vec<MatchBody> },
    Loop { code: Vec<Statement> },
    Return { expr: Option<Expression> },
    Break,
    Continue,
    Throw { expr: Expression },
    Variable { name: String, typ: Type, mutable: bool, expr: Expression },
    Expression { expr: Expression },
}

pub struct SwitchBody {
    target: String,
    code: Vec<Statement>,
}

pub struct MatchBody {
    target: String,
    params: Vec<String>,
    code: Vec<Statement>,
}