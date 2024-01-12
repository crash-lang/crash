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

pub struct Type {
    typ: TypeType,
    name: Option<String>,
}

/// We need this thing to differentiate a primitive type from an object like enum or class
pub enum TypeType {
    Obj,
    Generic { extends: Vec<Type> }, 
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
    Bool,
    Char,
}

impl Type {
    pub fn new(typ: TypeType, name: Option<String>) -> Self {
        Type { typ, name }
    }
}
