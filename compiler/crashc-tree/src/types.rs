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

use crashc_lexer::token::Token;

pub const CHAR_TYPE: Type = Type {
    name: "u8",
    unsigned: true,
    floating: false,
    size: 8,
};

pub const BOOLEAN_TYPE: Type = Type {
    name: "boolean",
    unsigned: true,
    floating: false,
    size: 1,
};

pub const I8_TYPE: Type = Type {
    name: "i8",
    unsigned: false,
    floating: false,
    size: 8,
};

pub const U8_TYPE: Type = Type {
    name: "u8",
    unsigned: true,
    floating: false,
    size: 8,
};

pub const I16_TYPE: Type = Type {
    name: "i16",
    unsigned: false,
    floating: false,
    size: 16,
};

pub const U16_TYPE: Type = Type {
    name: "u16",
    unsigned: true,
    floating: false,
    size: 16,
};

pub const I32_TYPE: Type = Type {
    name: "i32",
    unsigned: false,
    floating: false,
    size: 32,
};

pub const U32_TYPE: Type = Type {
    name: "u32",
    unsigned: true,
    floating: false,
    size: 32,
};

pub const I64_TYPE: Type = Type {
    name: "i64",
    unsigned: false,
    floating: false,
    size: 64,
};

pub const U64_TYPE: Type = Type {
    name: "u64",
    unsigned: true,
    floating: false,
    size: 64,
};

pub const I128_TYPE: Type = Type {
    name: "i128",
    unsigned: false,
    floating: false,
    size: 128,
};

pub const U128_TYPE: Type = Type {
    name: "u128",
    unsigned: true,
    floating: false,
    size: 128,
};

pub const F32_TYPE: Type = Type {
    name: "f32",
    unsigned: false,
    floating: true,
    size: 32,
};

pub const F64_TYPE: Type = Type {
    name: "f64",
    unsigned: false,
    floating: true,
    size: 64,
};

pub const NONE_TYPE: Type = Type {
    name: "_none_",
    unsigned: false,
    floating: false,
    size: 0,
};

pub const ALL_PRIMITIVE_TYPES: [Type; 15] = [
    CHAR_TYPE, BOOLEAN_TYPE,

    I8_TYPE,
    I16_TYPE,
    I32_TYPE,
    I64_TYPE,
    I128_TYPE,

    U8_TYPE,
    U16_TYPE,
    U32_TYPE,
    U64_TYPE,
    U128_TYPE,

    F32_TYPE,
    F64_TYPE,

    NONE_TYPE
];

pub fn type_by_token<'a>(token: Token) -> Type<'a> {
    for typ in ALL_PRIMITIVE_TYPES {
        if typ.name.eq(token.value()) {
            return typ;
        }
    }

    NONE_TYPE
}

#[derive(PartialEq, Eq)]
pub struct Type<'a> {
    name: &'a str,
    unsigned: bool,
    floating: bool,
    /// in bits
    size: u32
}

impl<'a> Type<'a> {
    pub fn new(name: &'a str, unsigned: bool, floating: bool, size: u32) -> Self {
        Self { name, unsigned, floating, size }
    }

    pub fn name(&self) -> &'a str {
        self.name
    }
    pub fn unsigned(&self) -> bool {
        self.unsigned
    }
    pub fn floating(&self) -> bool {
        self.floating
    }
    pub fn size(&self) -> u32 {
        self.size
    }
}

