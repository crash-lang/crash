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
 */

pub enum Access {
    Public,
    Protected,
    Internal,
    Private
}

pub enum Type {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    U128,
    I128,
    F32,
    F64,
    Boolean,
    Char,
    /// Name
    Identifier(String)
}

pub struct Expression {
    typ: Type,
    kind: Box<ExprKind>
}

pub enum ExprKind {
    // e.g. u32 i = 5
    // bool for mutable
    Assignment(Type, String, Box<Expression>, bool),
    // e.g. i = 4
    Reassignment(String, Expression),

    // e.g. x + 1
    Add(Expression, Expression),
    Subtract(Expression, Expression),
    Multiply(Expression, Expression),
    Divide(Expression, Expression),
    Modulus(Expression, Expression),

    // x == 1
    Equals(Expression, Expression),
    NotEquals(Expression, Expression),
    GreaterOrEqual(Expression, Expression),
    Greater(Expression, Expression),
    LessOrEqual(Expression, Expression),
    Less(Expression, Expression),

    // x < 10 && x > 2
    And(Expression, Expression),
    Or(Expression, Expression),

    // x == 1 ? return x : return 0;
    Tenary(Expression, Expression, Expression),

    FuncCall(String, Vec<Type>),

    // i32 x = match y {}
    Match(Expression, Vec<(String, Vec<Statement>)>)
}

pub enum Statement {
    If(Expression, Vec<Statement>),
    Switch(Expression, Vec<(String, Vec<Statement>)>),
    Loop(Vec<Statement>),
    // for (u32 x : ints) {}
    Foreach(Type, String, Vec<Statement>),
    Return(Expression),
    Expression(Expression)
}

pub struct FileTree  {
    package: String,
    name: String,
    imports: Vec<String>,
    fields: Vec<Field>,
    functions: Vec<Function>,
    class: Option<Box<Class>>
}

pub struct Field  {
    name: String,
    typ: Type,
    val: Option<Expression>,
    mutable: bool,
    access: Access
}

pub struct Function  {
    parent: FileTree,
    name: String,
    access: Access,
    ret_type: Type,
    // Param bool for mutable
    param: Vec<(Type, String, bool)>,
    code: Vec<Statement>
}

pub struct Class  {
    parent: Box<FileTree>,
    fields: Vec<Field>,
    constructors: Vec<Constructor>,
    methods: Vec<Method>
}

pub struct Method  {
    parent: Class,
    name: String,
    access: Access,
    ret_type: Type,
    // Param bool for mutable
    param: Vec<(Type, String, bool)>,
    code: Vec<Statement>
}

pub struct Constructor  {
    parent: Class,
    access: Access,
    // Param bool for mutable
    param: Vec<(Type, String, bool)>,
    code: Vec<Statement>
}

impl Expression {
    pub fn new(expr_type: Type, kind: ExprKind) -> Self {
        Expression { typ: expr_type, kind: Box::new(kind) }
    }
}

impl FileTree {
    pub fn new(package: String, name: String) -> Self {
        FileTree {
            package,
            name,
            imports: Vec::new(),
            fields: Vec::new(),
            functions: Vec::new(),
            class: None,
        }
    }

    pub fn set_class(&mut self, class: Class) {
        self.class = Some(Box::new(class));
    }

    pub fn remove_class(&mut self) {
        self.class = None;
    }

    pub fn add_field(&mut self, field: Field) {
        self.fields.push(field);
    }
}

