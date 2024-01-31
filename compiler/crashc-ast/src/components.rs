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

use crashc_lexer::token::{LiteralKind, Token};
use crate::misc::*;

pub struct Component <'a> {
    len: usize,
    content: Vec<&'a Token>
}

pub struct CrashModule<'a> {
    imports: Vec<Import<'a>>,
    fields: Vec<Field<'a>>,
    functions: Vec<Function<'a>>,
    classes: Vec<Class<'a>>,
    interfaces: Vec<Interface<'a>>,
    enums: Vec<Enum<'a>>
}

pub struct Import <'a> {
    path: Vec<String>,
    component: Component<'a>
}

/// Represents just a class
pub struct Class <'a> {
    name: Option<String>,
    generics: Vec<Generic>,
    fields: Vec<Field<'a>>,
    constructors: Vec<Constructor<'a>>,
    methods: Vec<Function<'a>>,
    component: Component<'a>
}

pub struct Interface <'a> {
    name: Option<String>,
    generics: Vec<Generic>,
    // Constructors have to be abstract in interfaces
    constructors: Vec<Constructor<'a>>,
    methods: Vec<Function<'a>>,
    component: Component<'a>
}

/// Represents just an enum
pub struct Enum <'a> {
    name: Option<String>,
    generics: Vec<Generic>,
    component: Component<'a>
}

pub struct EnumValue <'a> {
    name: String,
    parameters: Vec<Variable<'a>>,
    body: Option<Body<'a>>,
    component: Component<'a>
}

/// Represents a function or method
pub struct Function <'a> {
    access_modifier: AccessModifier,
    generics: Vec<Generic>,
    parameters: Vec<Variable<'a>>,
    return_type: Option<Type>,
    /// If none, the function is abstract
    body: Option<Body<'a>>,
    component: Component<'a>
}

pub struct Constructor <'a> {
    access_modifier: AccessModifier,
    parameters: Vec<Variable<'a>>,
    /// If none, the constructor is abstract
    body: Option<Body<'a>>,
    component: Component<'a>
}

pub struct Generic {
    name: String,
    implements: Vec<Type>
}

pub struct Variable <'a> {
    typ: Type,
    name: String,
    mutable: bool,
    component: Component<'a>
}

/// Used in classes or just modules
pub struct Field <'a> {
    access_modifier: AccessModifier,
    variable: Variable<'a>,
    component: Component<'a>
}

/// Represents code
pub struct Body <'a> {
    statements: Vec<Statement<'a>>,
    component: Component<'a>
}

pub struct Parameter <'a> {
    expressions: Expression,
    component: Component<'a>
}

pub enum Statement <'a> {
    Return { return_expression: Expression },
    Break,
    Continue,
    If { expression: Expression, body: Body<'a> },
    Loop { body: Body<'a> },
    For { variable: Variable<'a>, expression: Expression, body: Body<'a> },
    Match { expression: Expression, bodies: Vec<MatchBody<'a>> },
    Switch { expression: Expression, bodies: Vec<SwitchBody<'a>> },
    Expression {expression: Expression }
}

pub struct MatchBody <'a> {
    name: Option<String>,
    param: Vec<String>,
    default: bool,
    body: Body<'a>
}

pub struct SwitchBody <'a> {
    name: Option<String>,
    default: bool,
    body: Body<'a>
}

pub enum Expression {
    Assign { variable_name: String, expression: Box<Expression> },

    Equals { value: Box<Expression>, target: Box<Expression> },
    Not { expression: Box<Expression> },
    Greater {value: Box<Expression>, target: Box<Expression> },
    GreaterOrEquals { value: Box<Expression>, target: Box<Expression> },
    Less { value: Box<Expression>, target: Box<Expression> },
    LessOrEquals { value: Box<Expression>, target: Box<Expression> },
    And { value1: Box<Expression>, value2: Box<Expression> },
    Or { value1: Box<Expression>, value2: Box<Expression> },

    Add { value1: Box<Expression>, value2: Box<Expression> },
    Subtract { value1: Box<Expression>, value2: Box<Expression> },
    Multiply { value1: Box<Expression>, value2: Box<Expression> },
    Divide { value1: Box<Expression>, value2: Box<Expression> },
    Modulus { value1: Box<Expression>, value2: Box<Expression> },

    Ternary { expression: Box<Expression>, if_true: Box<Expression>, if_false: Box<Expression> },

    Array { values: Vec<Expression> },

    Variable { variable_name: String },

    Literal { literal_kind: LiteralKind }
}

impl <'a> Component <'a> {
    pub fn new(len: usize, content: Vec<&'a Token>) -> Self {
        Self { len, content }
    }

    pub fn len(&self) -> usize {
        self.len
    }
    pub fn content(&self) -> &Vec<&Token> {
        &self.content
    }
    pub fn set_len(&mut self, len: usize) {
        self.len = len;
    }
    pub fn set_content(&mut self, content: Vec<&'a Token>) {
        self.content = content;
    }
}

impl <'a> CrashModule<'a> {
    pub fn new() -> Self {
        Self {
            imports: vec![],
            fields: vec![],
            functions: vec![],
            classes: vec![],
            interfaces: vec![],
            enums: vec![],
        }
    }

    pub fn imports(&self) -> &Vec<Import<'a>> {
        &self.imports
    }
    pub fn fields(&self) -> &Vec<Field<'a>> {
        &self.fields
    }
    pub fn functions(&self) -> &Vec<Function<'a>> {
        &self.functions
    }
    pub fn classes(&self) -> &Vec<Class<'a>> {
        &self.classes
    }
    pub fn interfaces(&self) -> &Vec<Interface<'a>> {
        &self.interfaces
    }
    pub fn enums(&self) -> &Vec<Enum<'a>> {
        &self.enums
    }
    pub fn set_imports(&mut self, imports: Vec<Import<'a>>) {
        self.imports = imports;
    }
    pub fn set_fields(&mut self, fields: Vec<Field<'a>>) {
        self.fields = fields;
    }
    pub fn set_functions(&mut self, functions: Vec<Function<'a>>) {
        self.functions = functions;
    }
    pub fn set_classes(&mut self, classes: Vec<Class<'a>>) {
        self.classes = classes;
    }
    pub fn set_interfaces(&mut self, interfaces: Vec<Interface<'a>>) {
        self.interfaces = interfaces;
    }
    pub fn set_enums(&mut self, enums: Vec<Enum<'a>>) {
        self.enums = enums;
    }
}

impl <'a> Import <'a> {
    pub fn new(path: Vec<String>, component: Component<'a>) -> Self {
        Self { path, component }
    }


    pub fn component(&self) -> &Component<'a> {
        &self.component
    }
    pub fn set_path(&mut self, path: Vec<String>) {
        self.path = path;
    }
    pub fn set_component(&mut self, component: Component<'a>) {
        self.component = component;
    }
    pub fn path(&self) -> &Vec<String> {
        &self.path
    }
}

impl <'a> Class <'a> {
    pub fn new(name: Option<String>, generics: Vec<Generic>, fields: Vec<Field<'a>>, constructors: Vec<Constructor<'a>>, methods: Vec<Function<'a>>, component: Component<'a>) -> Self {
        Self { name, generics, fields, constructors, methods, component }
    }
}

impl <'a> Interface <'a> {
    pub fn new(name: Option<String>, generics: Vec<Generic>, constructors: Vec<Constructor<'a>>, methods: Vec<Function<'a>>, component: Component<'a>) -> Self {
        Self { name, generics, constructors, methods, component }
    }
}

impl <'a> Enum <'a> {
    pub fn new(name: Option<String>, generics: Vec<Generic>, component: Component<'a>) -> Self {
        Self { name, generics, component }
    }
}

impl <'a> EnumValue <'a> {
    pub fn new(name: String, parameters: Vec<Variable<'a>>, body: Option<Body<'a>>, component: Component<'a>) -> Self {
        Self { name, parameters, body, component }
    }
}

impl <'a> Function <'a> {
    pub fn new(access_modifier: AccessModifier, generics: Vec<Generic>, parameters: Vec<Variable<'a>>, return_type: Option<Type>, body: Option<Body<'a>>, component: Component<'a>) -> Self {
        Self { access_modifier, generics, parameters, return_type, body, component }
    }
}

impl <'a> Constructor <'a> {
    pub fn new(access_modifier: AccessModifier, parameters: Vec<Variable<'a>>, body: Option<Body<'a>>, component: Component<'a>) -> Self {
        Self { access_modifier, parameters, body, component }
    }
}

impl Generic {
    pub fn new(name: String, implements: Vec<Type>) -> Self {
        Self { name, implements }
    }
}

impl <'a> Variable <'a> {
    pub fn new(typ: Type, name: String, mutable: bool, component: Component<'a>) -> Self {
        Self { typ, name, mutable, component }
    }
}

impl <'a> Field <'a> {
    pub fn new(access_modifier: AccessModifier, variable: Variable<'a>, component: Component<'a>) -> Self {
        Self { access_modifier, variable, component }
    }
}

impl <'a> Body <'a> {
    pub fn new(statements: Vec<Statement<'a>>, component: Component<'a>) -> Self {
        Self { statements, component }
    }
}

impl <'a> Parameter <'a> {
    pub fn new(expressions: Expression, component: Component<'a>) -> Self {
        Self { expressions, component }
    }
}

impl <'a> MatchBody <'a> {
    pub fn new(name: Option<String>, param: Vec<String>, default: bool, body: Body<'a>) -> Self {
        Self { name, param, default, body }
    }
}

impl <'a> SwitchBody <'a> {
    pub fn new(name: Option<String>, default: bool, body: Body<'a>) -> Self {
        Self { name, default, body }
    }
}